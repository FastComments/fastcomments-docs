Successivamente, apri il file `view.php`. Puoi farlo con `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Usa i tasti freccia per scorrere fino in fondo. Cerca un testo che dica qualcosa del tipo:

```php
echo $OUTPUT->box_end();
```

Ora copiamo il codice che aggiunge il widget dei commenti:

[inline-code-attrs-start title = 'Codice commenti Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Usa i tasti freccia per posizionare il cursore prima della riga "box_end" e incolla.

Dovresti avere qualcosa del genere:

<div class="screenshot white-bg">
    <div class="title">Example</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle Example" />
</div>

Ora salva: 

1. Premi `ctrl+x`
2. Premi `y`
3. Premi `enter`

Questo è tutto!