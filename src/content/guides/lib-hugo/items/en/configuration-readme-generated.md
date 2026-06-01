All FastComments widget options are set under `[params.fastcomments]` in `hugo.toml`, and can be overridden per page in front matter under `[fastcomments]`. Precedence, lowest to highest: site params, page front matter, shortcode parameters.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

When neither `url` nor `urlId` is provided, `url` defaults to the page's permalink so comment threads stay tied to a stable URL.

### EU data residency

EU customers set `region = "eu"` to route the widget to `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Note on key casing

Hugo lowercases every key in `hugo.toml` and front matter, but the FastComments widgets require camelCase keys (`tenantId`, `hasDarkBackground`). This component restores the correct casing for every known top-level option automatically, so write options in their normal camelCase form. Keys nested inside an object value (for example the keys of a `translations` map, or fields of `pageReactConfig`) are not restored. Configure those through the FastComments dashboard customization UI instead.