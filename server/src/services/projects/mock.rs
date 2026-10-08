use super::{PAGE_SIZE, Project, ProjectPage, ProjectQuery, ProjectsService};
use anyhow::Result;
use async_trait::async_trait;

/// Serves a fixed set of sample projects so the app runs without an external API.
/// Keyword search matches any word against the project name, goal, and task.
/// Location is ignored.
pub struct MockProjectsService;

const SAMPLE_PROJECTS: &[(&str, &str, &str, &str)] = &[
    ("backyard-bird-count", "Backyard Bird Count", "Track bird populations across seasons.", "Count the birds you see in 15 minutes and log each species."),
    ("pollinator-watch", "Pollinator Watch", "Map where bees and butterflies feed.", "Photograph insects visiting flowers and note the plant."),
    ("night-sky-brightness", "Night Sky Brightness", "Measure light pollution worldwide.", "Compare the stars you can see against reference star charts."),
    ("frog-call-survey", "Frog Call Survey", "Monitor amphibian health in wetlands.", "Record frog calls at dusk and identify the species."),
    ("street-tree-census", "Street Tree Census", "Build a map of urban trees.", "Measure and photograph the trees on your block."),
    ("stream-water-quality", "Stream Water Quality", "Detect pollution in local streams.", "Test water samples for pH, nitrates, and clarity."),
    ("squirrel-behavior", "Squirrel Behavior Study", "Learn how squirrels adapt to cities.", "Observe squirrels for ten minutes and log their activity."),
    ("mushroom-mapper", "Mushroom Mapper", "Document fungi diversity.", "Photograph mushrooms and note the substrate they grow on."),
    ("cloud-observer", "Cloud Observer", "Check satellite cloud readings from the ground.", "Report cloud types at the time of a satellite pass."),
    ("ladybug-hunt", "Ladybug Hunt", "Track native and invasive ladybug species.", "Photograph ladybugs and submit the pictures for identification."),
    ("dog-cognition", "Dog Cognition Games", "Understand how dogs think and learn.", "Play short problem-solving games with your dog and record the results."),
    ("whale-spotter", "Whale Spotter", "Follow whale migration routes.", "Log whale sightings from shore or boat with time and place."),
];

fn sample_projects() -> Vec<Project> {
    SAMPLE_PROJECTS
        .iter()
        .map(|(slug, name, goal, task)| Project {
            name: Some(name.to_string()),
            link: Some(format!("https://acme-science.example/project/{slug}")),
            goal: Some(goal.to_string()),
            task: Some(task.to_string()),
            image: None,
        })
        .collect()
}

fn matches(project: &Project, words: &[String]) -> bool {
    let haystack = [&project.name, &project.goal, &project.task]
        .iter()
        .filter_map(|field| field.as_deref())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();

    words.is_empty() || words.iter().any(|word| haystack.contains(word.as_str()))
}

#[async_trait]
impl ProjectsService for MockProjectsService {
    async fn search(&self, query: &ProjectQuery) -> Result<ProjectPage> {
        let words: Vec<String> = query
            .keywords
            .as_deref()
            .unwrap_or("")
            .split(|c: char| !c.is_alphanumeric())
            .filter(|word| !word.is_empty())
            .map(str::to_lowercase)
            .collect();

        let found: Vec<Project> = sample_projects()
            .into_iter()
            .filter(|project| matches(project, &words))
            .collect();

        let page = query.page.max(1);
        let entities = found
            .iter()
            .skip((page - 1) * PAGE_SIZE)
            .take(PAGE_SIZE)
            .cloned()
            .collect();

        Ok(ProjectPage {
            total: Some(found.len()),
            page: Some(page),
            entities,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(keywords: &str, page: usize) -> ProjectQuery {
        ProjectQuery {
            keywords: Some(keywords.to_string()),
            page,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_mock_keyword_match() {
        let result = MockProjectsService.search(&query("bird", 1)).await.unwrap();

        assert_eq!(result.total, Some(1));
        assert_eq!(result.entities[0].name.as_deref(), Some("Backyard Bird Count"));
    }

    #[tokio::test]
    async fn test_mock_pagination() {
        let first = MockProjectsService.search(&query("", 1)).await.unwrap();
        let second = MockProjectsService.search(&query("", 2)).await.unwrap();

        assert_eq!(first.total, Some(SAMPLE_PROJECTS.len()));
        assert_eq!(first.entities.len(), PAGE_SIZE);
        assert_eq!(second.entities.len(), SAMPLE_PROJECTS.len() - PAGE_SIZE);
    }
}
