Η μικρότερη δυνατή επέκταση θα ήταν:

[inline-code-attrs-start title = 'Μια Απλή Επέκταση'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Για τους σκοπούς αυτού του παραδείγματος, αποθηκεύστε το ως `my-extension.js` και κάντε το διαθέσιμο στο `https://example.com/my-extension.min.js`.

Αυτή η επέκταση δεν κάνει τίποτα, εκτός από το ότι κατά τη φόρτωση ανακτά το αντικείμενο επέκτασης που δημιουργείται από τη βασική βιβλιοθήκη σχολίων.

Αυτό το `Extension` αντικείμενο είναι ένα singleton και δεν μοιράζεται με καμία άλλη επέκταση.

Στη συνέχεια, για να φορτώσουμε την επέκτασή μας, πρέπει να ενημερώσουμε το widget σχολίων για αυτήν. Για παράδειγμα:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Χρήση Προσαρμοσμένης Επέκτασης'; code-example-end]

Για λειτουργικά παραδείγματα, δείτε την επόμενη ενότητα.