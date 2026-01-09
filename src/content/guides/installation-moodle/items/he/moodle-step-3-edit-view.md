לאחר מכן, פתח את הקובץ `view.php`. אפשר לעשות זאת עם `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

השתמש במקשי החצים כדי לגלול למטה עד לתחתית. חפש טקסט שאומר משהו כמו:

```php
echo $OUTPUT->box_end();
```

כעת נעתיק את הקוד שמוסיף את ווידג'ט ההערות:

[inline-code-attrs-start title = 'קוד ההערות של Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

השתמש במקשי החצים כדי למקם את הסמן לפני השורה "box_end", והדבק.

כעת אמור להיראות משהו כזה:

<div class="screenshot white-bg">
    <div class="title">דוגמה</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="דוגמה של Moodle" />
</div>

עכשיו שמור: 

1. לחץ על `ctrl+x`
2. לחץ על `y`
3. לחץ על `enter`

זהו!