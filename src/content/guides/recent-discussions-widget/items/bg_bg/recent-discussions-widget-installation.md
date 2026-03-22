The Recent Discussions Widget shows pages on your site that have the most recent comment activity. Each entry displays the page title, last activity date, and total comment count. It automatically detects dark backgrounds and adjusts its styling accordingly.

## Basic Installation

[inline-code-attrs-start title = 'Инсталиране на приспособлението „Последни дискусии“'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Configuration Options

The `FastCommentsRecentDiscussionsV2` function accepts the following configuration options:

- **tenantId** (required): Your FastComments tenant ID
- **count** (optional): Number of pages to show. Default is `20`, max `100`
- **hasDarkBackground** (optional): Force dark mode styling. Auto-detected from the page background if not set

## Advanced Examples

### Custom Count

[inline-code-attrs-start title = 'Последни дискусии с потребителски брой'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### Force Dark Mode

[inline-code-attrs-start title = 'Последни дискусии в тъмен режим'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

---