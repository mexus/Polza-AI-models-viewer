#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortField {
    Name,
    Created,
    PromptPrice,
    CompletionPrice,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}
