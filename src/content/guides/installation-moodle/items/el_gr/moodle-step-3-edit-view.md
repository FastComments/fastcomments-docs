Στη συνέχεια, ανοίξτε το αρχείο `view.php`. Μπορείτε να το κάνετε με `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Χρησιμοποιήστε τα βελάκια για να μετακινηθείτε προς τα κάτω μέχρι το τέλος. Ψάξτε για κάποιο κείμενο που λέει κάτι σαν:

```php
echo $OUTPUT->box_end();
```

Τώρα ας αντιγράψουμε τον κώδικα που προσθέτει το widget σχολίων:

[inline-code-attrs-start title = 'Κώδικας σχολίων Moodle'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Χρησιμοποιήστε τα βελάκια για να τοποθετήσετε τον κέρσορα πριν από τη γραμμή "box_end" και επικολλήστε.

Θα πρέπει να έχετε κάτι σαν αυτό:

<div class="screenshot white-bg">
    <div class="title">Παράδειγμα</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Παράδειγμα Moodle" />
</div>

Τώρα αποθηκεύστε: 

1. Πατήστε `ctrl+x`
2. Πατήστε `y`
3. Πατήστε `enter`

Αυτό είναι!