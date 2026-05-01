The Recent Discussions widget displays the pages on your site with the most recent comment activity. It's useful for highlighting threads that are still being added to, so visitors can jump back into active conversations rather than landing on quiet pages.

## Options

- **Title** (optional): The heading shown above the list. Defaults to "Recent Discussions".
- **Count** (optional): How many discussions to show. Range 1 to 50. Defaults to 20.

## How to Add It

### Inside a Post or Page

In the block editor, add a **Shortcode** block and paste:

[inline-code-attrs-start title = 'Σύντομος κώδικας Recent Discussions'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

The `count` attribute accepts any value between 1 and 50.

### In a Sidebar or Footer (Classic Themes)

Go to **Appearance > Widgets** in your WordPress admin. From the block inserter, search for "FastComments" and choose **FastComments: Recent Discussions**. Drag it into a sidebar, header, or footer area, then configure the title and count from the widget panel.

### In a Block Theme (Full Site Editing)

Open the **Site Editor** under **Appearance > Editor**. Navigate to the template part where the widget should appear, insert a **Legacy Widget** block, and select **FastComments: Recent Discussions** from the dropdown.

## Troubleshooting

The widget only renders after FastComments setup is complete and a tenant ID is stored. If the widget area is blank, complete setup under **FastComments** in the WordPress admin and reload the page.

If discussion ordering looks stale, check that the underlying pages have finished syncing in the FastComments dashboard. The widget reads live data, so freshly imported comments may take a moment to surface.