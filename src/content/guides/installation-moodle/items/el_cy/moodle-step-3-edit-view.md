Στη συνέχεια, άνοιξε το αρχείο `view.php`. Μπορείς να το κάνεις με το `nano`:

```bash
sudo nano /var/www/html/moodle/mod/book/view.php
```

Χρησιμοποίησε τα βελάκια για να μετακινηθείς προς τα κάτω μέχρι το τέλος. Αναζήτησε κείμενο που λέει κάτι σαν:

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

Χρησιμοποίησε τα βελάκια για να τοποθετήσεις τον κέρσορα πριν από τη γραμμή "box_end" και επικόλλησε.

Θα πρέπει να έχεις κάτι σαν το παρακάτω:

<div class="screenshot white-bg">
    <div class="title">Παράδειγμα</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-moodle-result-code.png" alt="Παράδειγμα Moodle" />
</div>

Τώρα αποθήκευσε: 

1. Πάτησε `ctrl+x`
2. Πάτησε `y`
3. Πάτησε `enter`

Αυτό ήταν!