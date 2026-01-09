---
Η πιο μικρή δυνατή επέκταση θα ήταν:

[inline-code-attrs-start title = 'Μια Απλή Επέκταση'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Για χάρη αυτού του παραδείγματος, αποθηκεύστε το ως `my-extension.js`, και κάντε το διαθέσιμο στη διεύθυνση `https://example.com/my-extension.min.js`.

Αυτή η επέκταση δεν κάνει τίποτα, εκτός από το ότι κατά τη φόρτωση ανακτά το αντικείμενο της επέκτασης που δημιουργήθηκε από τη βασική βιβλιοθήκη σχολίων.

Αυτό το αντικείμενο `Extension` είναι singleton και δεν μοιράζεται με άλλες επεκτάσεις.

Στη συνέχεια, για να φορτώσουμε την επέκτασή μας, πρέπει να ενημερώσουμε το widget σχολίων γι' αυτή. Για παράδειγμα:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Για λειτουργικά παραδείγματα, δείτε την επόμενη ενότητα.

---