---
Next, open the `view.php` file. You can do this with `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Use the arrow keys to scroll down to the bottom. Look for some text that says something like:

```php
echo $OUTPUT->box_end();
```

Now let's copy the code that adds the comment widget:

[inline-code-attrs-start title = 'Moodle コメントコード'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

if ($id) {
    $url_decoded = str_replace('&amp;', '&', $PAGE->url);
    $users_picture_obj = new user_picture($USER);
    $users_picture_url = $users_picture_obj->get_url($PAGE);
    
    $simple_sso_json = json_encode($USER && $USER->username !== 'guest' ? array(
        "username" => $USER->firstname . $USER->lastname,
        "email" => $USER->email,
        "avatar" => $users_picture_url->out(false)
    ) : array(
        "loginURL" => '/login/index.php'
    ));
    
    echo "<script src=\"https://cdn-eu.fastcomments.com/js/embed-v2.min.js\"></script>
    <div id=\"fastcomments-widget\"></div>
    <script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
            tenantId: 'demo',
            simpleSSO: $simple_sso_json,
            urlId: $id,
            url: '$url_decoded'
        });
    </script>";
}
[inline-code-end]

Use the arrow keys to position your cursor before the "box_end" line, and paste.

You should have something like this:

<div class="screenshot white-bg">
    <div class="title">例</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle の例" />
</div>

Now save: 

1. Press `ctrl+x`
2. Press `y`
3. Press `enter`

That's it!

---