接下来，打开 `view.php` 文件。你可以使用 `nano`：

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

使用方向键向下滚动到文件底部。查找类似如下的文本：

```php
echo $OUTPUT->box_end();
```

现在复制添加评论小部件的代码：

[inline-code-attrs-start title = 'Moodle 评论 代码'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

使用方向键将光标定位在 "box_end" 行之前，然后粘贴。

你应该会得到如下内容：

<div class="screenshot white-bg">
    <div class="title">示例</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle 示例" />
</div>

现在保存： 

1. 按 `ctrl+x`
2. 按 `y`
3. 按 `enter`

就这样！