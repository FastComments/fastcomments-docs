The Recent Discussions widget displays the pages on your site with the most recent comment activity. It's useful for highlighting threads that are still being added to, so visitors can jump back into active conversations rather than landing on quiet pages.

## Options

- **标题**（可选）： 列表上方显示的标题。默认值为 "最近讨论"。
- **数量**（可选）： 要显示多少个讨论。范围 1 到 50。默认值为 20。

## How to Add It

### Inside a Post or Page

In the block editor, add a **Shortcode** block and paste:

[inline-code-attrs-start title = '近期讨论 短代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

The `count` attribute accepts any value between 1 and 50.

### In a Sidebar or Footer (Classic Themes)

Go to **外观 > 小工具** in your WordPress admin. From the block inserter, search for "FastComments" and choose **FastComments: Recent Discussions**. Drag it into a sidebar, header, or footer area, then configure the title and count from the widget panel.

### In a Block Theme (Full Site Editing)

Open the **站点编辑器** under **外观 > 编辑器**. Navigate to the template part where the widget should appear, insert a **传统小工具** block, and select **FastComments: Recent Discussions** from the dropdown.

## Troubleshooting

The widget only renders after FastComments setup is complete and a tenant ID is stored. If the widget area is blank, complete setup under **FastComments** in the WordPress admin and reload the page.

If discussion ordering looks stale, check that the underlying pages have finished syncing in the FastComments 仪表板. The widget reads live data, so freshly imported comments may take a moment to surface.