Zatim otvorite datoteku `view.php`. To možete učiniti pomoću `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Koristite tipke sa strelicama da se pomaknete prema dnu. Potražite tekst koji kaže otprilike ovako:

```php
echo $OUTPUT->box_end();
```

Sada kopirajmo kod koji dodaje widget za komentare:

[inline-code-attrs-start title = 'Kod komentara za Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Koristite tipke sa strelicama da pozicionirate kursor prije retka "box_end" i zalijepite.

Trebali biste imati nešto poput ovoga:

<div class="screenshot white-bg">
    <div class="title">Primjer</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Primjer Moodle" />
</div>

Sada spremite: 

1. Pritisnite `ctrl+x`
2. Pritisnite `y`
3. Pritisnite `enter`

To je to!