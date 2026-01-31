Ensuite, ouvrez le fichier `view.php`. Vous pouvez le faire avec `nano` :

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Utilisez les touches fléchées pour faire défiler jusqu'en bas. Recherchez un texte qui ressemble à :

```php
echo $OUTPUT->box_end();
```

Maintenant, copions le code qui ajoute le widget de commentaires :

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

Utilisez les touches fléchées pour positionner votre curseur avant la ligne "box_end", puis collez.

Vous devriez obtenir quelque chose comme ceci :

<div class="screenshot white-bg">
    <div class="title">Example</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle Example" />
</div>

Maintenant, enregistrez : 

1. Appuyez sur `ctrl+x`
2. Appuyez sur `y`
3. Appuyez sur `enter`

C'est tout !