---
Sonra, `view.php` dosyasını açın. Bunu `nano` ile yapabilirsiniz:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Aşağı kaydırmak için ok tuşlarını kullanın. Şunun gibi bir metin arayın:

```php
echo $OUTPUT->box_end();
```

Şimdi yorum bileşenini ekleyen kodu kopyalayalım:

[inline-code-attrs-start title = 'Moodle Yorum Kodu'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Ok tuşlarını kullanarak imlecinizi "box_end" satırının önüne getirin ve yapıştırın.

Şunun gibi bir şey görmelisiniz:

<div class="screenshot white-bg">
    <div class="title">Example</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle Example" />
</div>

Şimdi kaydedin: 

1. Press `ctrl+x`
2. Press `y`
3. Press `enter`

Hepsi bu!

---