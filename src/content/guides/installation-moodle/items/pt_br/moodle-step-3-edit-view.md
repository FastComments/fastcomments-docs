Em seguida, abra o arquivo `view.php`. Você pode fazer isso com `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Use as teclas de seta para descer até o final. Procure por um texto que diga algo como:

```php
echo $OUTPUT->box_end();
```

Agora vamos copiar o código que adiciona o widget de comentários:

[inline-code-attrs-start title = 'Código de Comentários do Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Use as teclas de seta para posicionar o cursor antes da linha "box_end" e cole.

Você deve ter algo parecido com isto:

<div class="screenshot white-bg">
    <div class="title">Exemplo</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Exemplo do Moodle" />
</div>

Agora salve: 

1. Pressione `ctrl+x`
2. Pressione `y`
3. Pressione `enter`

Pronto!