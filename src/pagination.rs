use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub from: usize,
    pub to: usize,
    pub page: u32,
    pub total: u32,
    pub items: Vec<PaginationItem>,
    pub next: Option<String>,
    pub prev: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationItem {
    pub url: String,
    pub is_active: bool,
    pub label: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPagination {
    pub page: Option<u32>,
}

impl Pagination {
    pub fn new(total_items: usize, page: Option<u32>) -> Self {
        let total = total_items.div_ceil(10) as u32;
        let page = page.unwrap_or(1).min(total).max(1);

        // Calculate offset based on page
        let offset = ((page - 1) * 10) as usize;

        let mut pagination = Self {
            from: offset,
            to: (offset + 10).min(total_items),
            total,
            page,
            next: None,
            prev: None,
            items: vec![],
        };

        // Insert 'next' link if there is a next page
        if total > page {
            pagination.next = Some(format!("/users?page={}", page + 1));
        }

        // Insert 'prev' link if there is a previous page
        if page > 1 {
            pagination.prev = Some(format!("/users?page={}", page - 1));
        }

        // Calculate the starting point for pagination items
        let count_start = (page.checked_sub(2).unwrap_or(page)).min(total).max(1);

        // Generate pagination items
        let items: Vec<PaginationItem> = (0..3)
            .map(|i| {
                let label = count_start + i;
                PaginationItem {
                    label: format!("{}", label),
                    url: format!("/users?page={}", label),
                    is_active: label == page,
                }
            })
            .collect();

        pagination.items = items;

        return pagination
    }
}