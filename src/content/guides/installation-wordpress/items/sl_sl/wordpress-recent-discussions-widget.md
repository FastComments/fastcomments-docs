Pripomoček Recent Discussions prikazuje strani na vašem spletnem mestu z najnovejšo aktivnostjo komentarjev. Uporaben je za izpostavitev tem, v katere se še dodaja, tako da lahko obiskovalci skočijo nazaj v aktivne pogovore namesto, da pristanejo na mirnih straneh.

## Options

- **Title** (optional): The heading shown above the list. Defaults to "Nedavne razprave".
- **Count** (optional): How many discussions to show. Range 1 to 50. Defaults to 20.

## How to Add It

### Inside a Post or Page

In the block editor, add a **Shortcode** block and paste:

[inline-code-attrs-start title = 'Shortcode nedavnih razprav'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

The `count` attribute accepts any value between 1 and 50.

### In a Sidebar or Footer (Classic Themes)

Go to **Videz > Pripomočki** in your WordPress admin. From the block inserter, search for "FastComments" and choose **FastComments: Nedavne razprave**. Drag it into a sidebar, header, or footer area, then configure the title and count from the widget panel.

### In a Block Theme (Full Site Editing)

Open the **Urejevalnik mesta** under **Videz > Urejevalnik**. Navigate to the template part where the widget should appear, insert a **Legacy Widget** block, and select **FastComments: Nedavne razprave** from the dropdown.

## Troubleshooting

The widget only renders after FastComments setup is complete and a tenant ID is stored. If the widget area is blank, complete setup under **FastComments** in the WordPress admin and reload the page.

If discussion ordering looks stale, check that the underlying pages have finished syncing in the FastComments dashboard. The widget reads live data, so freshly imported comments may take a moment to surface.