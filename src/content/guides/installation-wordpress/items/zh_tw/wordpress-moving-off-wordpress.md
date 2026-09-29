If you are moving your site off of WordPress and want FastComments on the new site, you do not need the WordPress plugin. Export your comments from
WordPress, then upload the file on the [Import page](https://fastcomments.com/auth/my-account/manage-data/import) in the FastComments dashboard.

We support two WordPress export formats.

### WordPress XML（Recommended）

This is the file from WordPress's built-in exporter, so no extra plugin is needed.

1. In your WordPress admin, go to `Tools -> Export`.
2. Select `All content` and click `Download Export File`.
3. On the FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import), select `WordPress (.xml)` and upload the file.

Each comment is tied to the URL of the post it was left on, which is already in the file.

The import keeps the author name, email, and website, the date, the content, reply threading, and whether the comment was approved. Commenter avatars are
brought over from Gravatar. Votes are not part of this format.

### WordPress CSV

This is the file from [WebToffee's WordPress Comments Import & Export plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Install the plugin in your WordPress admin and export your comments as CSV.
2. Replace each `comment_post_ID` value with the post's URL.
3. On the FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import), select `WordPress (.csv)` and upload the file.

Each comment is tied to the `comment_post_ID` column. WordPress fills this column with the post ID, and your new site does not have WordPress post IDs,
so step 2 replaces it with the URL.

The import keeps the author name, email, and website, the date, the content, reply threading, and whether the comment was approved. Commenter avatars are
brought over from Gravatar. It also keeps WordPress's spam flag, and wpDiscuz likes and dislikes when the file includes them.

### Matching Comments to Your New Pages

If your new site keeps the same URLs as your WordPress site, the comments show up on the matching pages with no extra setup.

If the domain changes, run the [Domain Migration tool](/guide-migrations.html#migrating-domains) after the import. If individual page URLs change, you can
[migrate each page](/guide-migrations.html#migrating-pages) from its old URL to the new one.

For bulk page migrations, such as removing the domain from the value you pass to the comment widget's [urlId](/guide-customizations-and-configuration.html#url-id)
field, [open a support ticket](https://fastcomments.com/auth/my-account/help) and we will handle it for you.

### Before You Switch

You can run the import as many times as you like. Re-importing the same file [does not create duplicates](/guide-migrations.html#importing-data), so you can
import once to test the new site, then import again with your latest comments right before switching over.

For export files larger than 1GB, [reach out to support](https://fastcomments.com/auth/my-account/help).

To add FastComments to your new site, see the [Installation guide](/guide-installation.html).