use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Response<T> {
    data: T,
}

#[derive(Deserialize)]
struct ActiveDailyCodingChallengeQuestion {
    #[serde(rename = "activeDailyCodingChallengeQuestion")]
    problem: LeetCodeProblem,
}

#[derive(Deserialize)]
struct QuestionContent {
    question: Description,
}

#[derive(Deserialize)]
struct Description {
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct LeetCodeProblem {
    pub link: String,
    #[serde(rename = "question")]
    pub properties: LeetCodeProblemProprties,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeetCodeProblemProprties {
    pub difficulty: String,
    pub title: String,
    pub title_slug: String,
    pub topic_tags: Vec<LeetCodeProblemTopic>,
}

#[derive(Debug, Deserialize)]
pub struct LeetCodeProblemTopic {
    pub name: String,
}

pub struct LeetCodeApi<'a> {
    api_endpoint: &'a str,
}

impl<'a> LeetCodeApi<'a> {
    pub fn new(api_endpoint: &'a str) -> Self {
        Self { api_endpoint }
    }

    /// Fetch daily LeetCode problem via HTTP
    ///
    /// # Errors
    ///
    /// if failed to connect to LeetCode endpoint or
    /// failed to fetch data from LeetCode or
    /// failed to parse response
    pub async fn fetch_daily_problem(&self) -> anyhow::Result<LeetCodeProblem> {
        // Graphql query to fetch daily problem
        const DAILY_PROBLEM_QUERY: &str = r#"
            query questionOfToday { 
                activeDailyCodingChallengeQuestion {
                    link
                    question {
                        difficulty
                        title
                        titleSlug
                        topicTags {
                            name
                        }
                    }
                }
            }"#;

        let response = Client::builder()
            .build()?
            .post(self.api_endpoint)
            .json(&json!({ "query": DAILY_PROBLEM_QUERY }))
            .send()
            .await?
            .json::<Response<ActiveDailyCodingChallengeQuestion>>()
            .await?;

        Ok(response.data.problem)
    }

    /// Fetch LeetCode problem desciption by problem title via HTTP
    ///
    /// # Errors
    ///
    /// if failed to connect to LeetCode endpoint or
    /// failed to fetch data from LeetCode or
    /// failed to parse response
    pub async fn fetch_problem_description(&self, title_slug: &str) -> anyhow::Result<String> {
        // Graphql query to fetch problem description
        const PROBLEM_DESCRIPTION_QUERY: &str = r#"
            query questionContent($titleSlug: String!) {
                question(titleSlug: $titleSlug) {
                    content
                }
            }
        "#;

        let response = Client::builder()
            .build()?
            .post(self.api_endpoint)
            .json(&json!({
                "query": PROBLEM_DESCRIPTION_QUERY,
                "variables": {
                    "titleSlug": title_slug
                },
                "operationName": "questionContent"
            }))
            .send()
            .await?
            .json::<Response<QuestionContent>>()
            .await?;

        Ok(response.data.question.content)
    }
}
