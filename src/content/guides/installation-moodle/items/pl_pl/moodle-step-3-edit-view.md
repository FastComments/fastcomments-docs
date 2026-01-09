Następnie otwórz plik `view.php`. Możesz to zrobić za pomocą `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Użyj klawiszy strzałek, aby przewinąć na dół. Poszukaj linii z tekstem podobnym do:

```php
echo $OUTPUT->box_end();
```

Teraz wkleimy kod, który dodaje widżet komentarzy:

[inline-code-attrs-start title = 'Kod komentarzy Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Użyj klawiszy strzałek, aby ustawić kursor przed linią "box_end" i wklej.

Powinieneś mieć coś takiego:

<div class="screenshot white-bg">
    <div class="title">Example</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle Example" />
</div>

Teraz zapisz: 

1. Naciśnij `ctrl+x`
2. Naciśnij `y`
3. Naciśnij `enter`

To wszystko!