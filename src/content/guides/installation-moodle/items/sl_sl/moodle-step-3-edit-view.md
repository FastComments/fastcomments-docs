Nato odprite datoteko `view.php`. To lahko naredite z `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Uporabite puščične tipke, da se pomaknete do dna. Poiščite besedilo, ki je nekaj takega:

```php
echo $OUTPUT->box_end();
```

Zdaj kopirajmo kodo, ki doda vtičnik za komentarje:

[inline-code-attrs-start title = 'Koda komentarjev za Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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
    
    echo "<script async src=\"https://cdn.fastcomments.com/js/embed-v2-async.min.js\"></script>
    <div id=\"fastcomments-widget\"></div>
    <script>
    window.fcConfigs = [{
            target: '#fastcomments-widget',
            tenantId: 'demo',
            simpleSSO: $simple_sso_json,
            urlId: $id,
            url: '$url_decoded'
        }];
    </script>";
}
[inline-code-end]

S puščičnimi tipkami postavite kazalec pred vrstico "box_end" in prilepite.

Moral bi imeti nekaj takega:

<div class="screenshot white-bg">
    <div class="title">Primer</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Primer Moodle" />
</div>

Zdaj shranite: 

1. Pritisnite `ctrl+x`
2. Pritisnite `y`
3. Pritisnite `enter`

To je vse!