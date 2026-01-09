Η αλληλεπίδραση με το `Extension` είναι απλή, καθώς απλώς ορίζουμε αναφορές προς συναρτήσεις που θέλουμε να κληθούν.

Για να επεκτείνουμε το προηγούμενο παράδειγμα, ας υποθέσουμε ότι θέλουμε να προσθέσουμε HTML στο πάνω μέρος κάθε σχολίου:

[inline-code-attrs-start title = 'Ένα Απλό Extension - Συνέχεια'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    extension.commentFilter = function(comment, html) {
        return `<h3>The user's name is ${comment.commenterName}!</h3>` + html;
    }
})();
[inline-code-end]

Όποτε επιστρέφετε HTML σαν αυτή, θα συγχωνευτεί στο UI μέσω ενός αλγορίθμου dom-diffing.

#### Χειροκίνητη ενεργοποίηση της επανα-απόδοσης ενός σχολίου

Μπορούμε να περιμένουμε το αρχικό φόρτωμα της σελίδας και να επανα-αποδώσουμε χειροκίνητα ένα σχόλιο καλώντας την `reRenderComment`:

[inline-code-attrs-start title = 'Επανα-απόδοση ενός σχολίου'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    let renderCount = 0;

    extension.commentFilter = function(comment, html) {
        renderCount++;
        return `<h3>The render count is ${renderCount}!</h3>` + html;
    }

    extension.onInitialRenderComplete = function() {
        setInterval(function() {
            extension.reRenderComment(extension.commentsById[Object.keys(extension.commentsById)[0]], function renderDone() {
                console.log('Comment re-render done.');
            });
        }, 2000); // timeout not required, just an example.
    }
})();
[inline-code-end]

---