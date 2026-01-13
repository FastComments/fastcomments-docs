---
Η μικρότερη δυνατή επέκταση θα ήταν:

[inline-code-attrs-start title = 'Μια Απλή Επέκταση'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Για το παράδειγμα αυτό, αποθηκεύστε το ως `my-extension.js`, και κάντε το διαθέσιμο στο `https://example.com/my-extension.min.js`.

Αυτή η επέκταση δεν κάνει τίποτα, εκτός από το ότι κατά τη φόρτωση ανακτά το αντικείμενο επέκτασης που δημιουργείται από τη βασική βιβλιοθήκη σχολίων.

This `Extension` object is a singleton and is not shared with any other extensions.

Στη συνέχεια, για να φορτώσουμε την επέκτασή μας, πρέπει να ενημερώσουμε το widget σχολίων γι' αυτήν. Για παράδειγμα:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Για λειτουργικά παραδείγματα, δείτε την επόμενη ενότητα.

---