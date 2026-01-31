接下來，打開 `view.php` 檔案。你可以使用 `nano` 執行此操作：

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

使用方向鍵向下捲動到最底部。尋找類似以下的文字：

```php
echo $OUTPUT->box_end();
```

現在讓我們複製新增留言元件的程式碼：

[inline-code-attrs-start title = 'Moodle 評論程式碼'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

使用方向鍵將游標定位在 "box_end" 行之前，然後貼上。

你應該會得到類似以下內容：

<div class="screenshot white-bg">
    <div class="title">範例</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle 範例" />
</div>

現在儲存： 

1. 按 `ctrl+x`
2. 按 `y`
3. 按 `enter`

就是這樣！