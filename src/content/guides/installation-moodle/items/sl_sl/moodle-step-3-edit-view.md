---
Nato odprite datoteko `view.php`. To lahko naredite z `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Uporabite puščice na tipkovnici, da se pomaknete navzdol do dna. Poiščite nekaj besedila, ki pravi nekaj takega:

```php
echo $OUTPUT->box_end();
```

Zdaj kopirajmo kodo, ki doda pripomoček za komentarje:

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

Uporabite puščice, da postavite kurzor pred vrstico "box_end" in prilepite.

Morali bi imeti nekaj takega:

<div class="screenshot white-bg">
    <div class="title">Primer</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Primer Moodle" />
</div>

Zdaj shranite: 

1. Pritisnite `ctrl+x`
2. Pritisnite `y`
3. Pritisnite `enter`

To je vse!

---