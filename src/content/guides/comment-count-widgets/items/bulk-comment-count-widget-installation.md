The Bulk Comment Count Widget is designed for efficiently displaying comment counts for multiple pages on the same page. Instead of making individual API calls for each comment count, this widget batches requests for optimal performance.

## Basic Installation

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsCommentCountBulk({
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## How It Works

The bulk widget works by:

1. Scanning the page for elements with the class `fast-comments-count`
2. Reading the `data-fast-comments-url-id` attribute from each element
3. Batching API requests to fetch multiple comment counts efficiently
4. Updating each element with the appropriate comment count

## Configuration Options

The `FastCommentsCommentCountBulk` function accepts the following configuration options:

- **tenantId** (required): Your FastComments tenant ID
- **apiHost** (optional): Custom API host if you're using a self-hosted instance

## Real-World Example

Here's a practical example showing how you might use the bulk widget in a blog post listing:

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 | 
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 | 
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 | 
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsCommentCountBulk({
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Performance Considerations

The bulk widget automatically optimizes performance by:

- **Batching requests**: Multiple comment counts are fetched in a single API call
- **Request size limits**: Requests are automatically split if the URL list becomes too large (over 1,000 characters)
- **Deduplication**: Multiple elements with the same `data-fast-comments-url-id` share the same count

## Multiple Elements with Same URL ID

You can have multiple elements on the page with the same `data-fast-comments-url-id`. They will all be updated with the same count:

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsCommentCountBulk({
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Localization

The bulk widget automatically formats comment counts based on your FastComments language settings. It provides appropriate text for:

- Zero comments
- One comment  
- Multiple comments

## When to Use Bulk vs Single Widget

**Use the Bulk Widget when:**
- You have multiple comment counts on the same page
- You're displaying a list of posts/articles with comment counts
- Performance is important (reduces API calls)

**Use the Single Widget when:**
- You only need one comment count on the page
- You need live updates (the single widget supports real-time updates)
- You want more control over individual widget behavior