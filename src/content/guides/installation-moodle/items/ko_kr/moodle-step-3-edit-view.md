다음으로 `view.php` 파일을 엽니다. `nano`로 열 수 있습니다:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

화살표 키를 사용하여 맨 아래로 스크롤하세요. 다음과 유사한 텍스트를 찾으세요:

```php
echo $OUTPUT->box_end();
```

이제 댓글 위젯을 추가하는 코드를 복사합시다:

[inline-code-attrs-start title = 'Moodle 댓글 코드'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

화살표 키로 커서를 "box_end" 줄 이전에 위치시키고 붙여넣으세요.

다음과 비슷해야 합니다:

<div class="screenshot white-bg">
    <div class="title">예제</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Moodle 예제" />
</div>

이제 저장: 

1. Press `ctrl+x`
2. Press `y`
3. Press `enter`

그게 전부입니다!