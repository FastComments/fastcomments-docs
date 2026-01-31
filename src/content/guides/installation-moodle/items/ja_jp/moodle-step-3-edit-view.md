---
次に、`view.php` ファイルを開きます。`nano` を使って開けます：

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

矢印キーを使って下までスクロールします。次のようなテキストを探してください：

```php
echo $OUTPUT->box_end();
```

次に、コメントウィジェットを追加するコードをコピーします：

[inline-code-attrs-start title = 'Moodle コメントコード'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

矢印キーを使ってカーソルを "box_end" 行の前に移動し、貼り付けます。

次のようになっているはずです：

<div class="screenshot white-bg">
    <div class="title">例</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle の例" />
</div>

保存するには：

1. `ctrl+x` を押す
2. `y` を押す
3. `enter` を押す

これで完了です！

---