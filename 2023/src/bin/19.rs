use anyhow::Result;
use core::panic;
use std::{collections::HashMap, fs};

#[derive(Debug)]
enum RuleResult {
    Approve,
    Reject,
    Redirect(String),
}

impl RuleResult {
    fn new(s: &str) -> Self {
        if s == "A" {
            Self::Approve
        } else if s == "R" {
            Self::Reject
        } else {
            Self::Redirect(s.to_string())
        }
    }
}

#[derive(Debug)]
struct Rule {
    field: Option<usize>,
    less: bool,
    val: usize,
    res: RuleResult,
}

impl Rule {
    fn new(s: &str) -> Self {
        if let Some((cond, res)) = s.split_once(':') {
            Self {
                field: Some(field_to_num(cond.chars().next().unwrap())),
                less: cond.chars().nth(1).unwrap() == '<',
                val: cond
                    .chars()
                    .skip(2)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
                res: RuleResult::new(res),
            }
        } else {
            Self {
                field: None,
                less: false,
                val: 0,
                res: RuleResult::new(s),
            }
        }
    }
    fn break_range(&self, mut range: RatingRange) -> (RatingRange, RatingRange) {
        let mut cloned = range;
        if let Some(field) = self.field {
            if self.less {
                range[field].1 = range[field].1.min(self.val - 1);
                cloned[field].0 = cloned[field].0.max(self.val);
            } else {
                range[field].0 = range[field].0.max(self.val + 1);
                cloned[field].1 = cloned[field].1.min(self.val);
            }
        }
        (range, cloned)
    }
}

type Rating = [usize; 4];
type RatingRange = [(usize, usize); 4];

fn count_ratings_in_range(range: &RatingRange) -> usize {
    range.iter().fold(1, |acc, sub_range| {
        acc * ((sub_range.1 + 1).saturating_sub(sub_range.0))
    })
}

fn field_to_num(rating: char) -> usize {
    match rating {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("invalid rating"),
    }
}

type Workflow = Vec<Rule>;

fn eval_rating<'a>(workflow: &'a Workflow, rating: &Rating) -> &'a RuleResult {
    for rule in workflow {
        if let Some(field) = rule.field {
            if (rule.less && rating[field] < rule.val) || (!rule.less && rating[field] > rule.val) {
                return &rule.res;
            }
        } else {
            return &rule.res;
        }
    }
    panic!("no rule matched");
}

fn eval_rating_all(workflows: &HashMap<String, Workflow>, rating: &Rating) -> bool {
    let mut current = workflows.get("in").unwrap();
    loop {
        let res = eval_rating(current, rating);
        match res {
            RuleResult::Approve => return true,
            RuleResult::Reject => return false,
            RuleResult::Redirect(s) => current = workflows.get(s).unwrap(),
        }
    }
}

fn part_one(workflows: HashMap<String, Workflow>, ratings: Vec<Rating>) -> usize {
    ratings
        .iter()
        .filter(|r| eval_rating_all(&workflows, r))
        .map(|r| r.iter().sum::<usize>())
        .sum()
}

fn eval_range(
    current_range: RatingRange,
    current_wf: &Workflow,
    workflows: &HashMap<String, Workflow>,
) -> usize {
    current_wf
        .iter()
        .fold((0, current_range), |(sum, range), rule| {
            let (new_current_range, next_range) = rule.break_range(range);
            match &rule.res {
                RuleResult::Approve => {
                    (sum + count_ratings_in_range(&new_current_range), next_range)
                }
                RuleResult::Reject => (sum, next_range),
                RuleResult::Redirect(next_wf) => (
                    sum + eval_range(
                        new_current_range,
                        workflows.get(next_wf).unwrap(),
                        workflows,
                    ),
                    next_range,
                ),
            }
        })
        .0
}

fn part_two(workflows: HashMap<String, Workflow>) -> usize {
    eval_range([(1, 4000); 4], workflows.get("in").unwrap(), &workflows)
}

fn parse(file_contents: String) -> (HashMap<String, Workflow>, Vec<Rating>) {
    let (workflows, ratings) = file_contents.split_once("\n\n").unwrap();

    let workflows = workflows.lines().fold(HashMap::new(), |mut acc, line| {
        let (name, rest) = line.split_once('{').unwrap();
        let rules: Vec<Rule> = rest
            .trim_end_matches('}')
            .split(',')
            .map(Rule::new)
            .collect();
        acc.insert(name.to_string(), rules);
        acc
    });
    let braces: &[_] = &['{', '}'];
    let ratings = ratings
        .lines()
        .map(|line| {
            line.trim_matches(braces)
                .split(',')
                .map(|field| field.split_once('=').unwrap().1.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap()
        })
        .collect();
    (workflows, ratings)
}

fn main() -> Result<()> {
    let file_contents = fs::read_to_string("input/19.txt")?;

    let (workflows, ratings) = parse(file_contents);

    // println!("{}", part_one(workflows, ratings));
    println!("{}", part_two(workflows));
    Ok(())
}
