Ensuite, ouvrez le fichier `view.php`. Vous pouvez le faire avec `nano` :

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Utilisez les touches fléchées pour faire défiler jusqu'en bas. Recherchez un texte qui ressemble à :

```php
echo $OUTPUT->box_end();
```

Copions maintenant le code qui ajoute le widget de commentaires :

[inline-code-attrs-start title = 'Code des commentaires Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Utilisez les touches fléchées pour placer votre curseur avant la ligne "box_end", puis collez.

Vous devriez avoir quelque chose comme ceci :

<div class="screenshot white-bg">
    <div class="title">Exemple</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Exemple Moodle" />
</div>

Enregistrez maintenant : 

1. Press `ctrl+x`
2. Press `y`
3. Press `enter`

C'est tout !