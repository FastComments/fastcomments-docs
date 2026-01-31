---
A continuación, abra el archivo `view.php`. Puede hacerlo con `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Use las teclas de flecha para desplazarse hasta el final. Busque un texto que diga algo como:

```php
echo $OUTPUT->box_end();
```

Ahora copie el código que añade el widget de comentarios:

[inline-code-attrs-start title = 'Código de comentarios de Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Use las teclas de flecha para colocar el cursor antes de la línea "box_end" y pegue.

Debería tener algo así:

<div class="screenshot white-bg">
    <div class="title">Ejemplo</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Ejemplo de Moodle" />
</div>

Ahora guarde: 

1. Press `ctrl+x`
2. Press `y`
3. Press `enter`

¡Eso es todo!

---