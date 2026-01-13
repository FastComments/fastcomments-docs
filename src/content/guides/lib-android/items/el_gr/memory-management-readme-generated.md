### Πρόληψη διαρροών μνήμης

Για να αποφύγετε διαρροές μνήμης κατά τη χρήση των FastComments views σε Activities ή Fragments, καλέστε πάντα την `cleanup()` όταν η προβολή δεν χρειάζεται πλέον:

#### Σε Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Εκκαθάριση των προβολών FastComments για να αποφευχθούν διαρροές μνήμης
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### Σε Fragments:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // Εκκαθάριση των προβολών FastComments όταν η προβολή του fragment καταστραφεί
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Επιπλέον εκκαθάριση όταν το fragment καταστραφεί
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Όταν γίνεται εναλλαγή Fragments:
```java
// Πριν αντικαταστήσετε ή αφαιρέσετε ένα fragment που περιέχει προβολές FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Στη συνέχεια, προχωρήστε με τη συναλλαγή fragment
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Σημαντικό**: Καλείτε πάντα τις μεθόδους `cleanup()` για να αποτρέψετε διαρροές μνήμης, ιδιαίτερα όταν:
- Activities καταστρέφονται
- Οι προβολές των Fragments καταστρέφονται
- Γίνεται εναλλαγή μεταξύ fragments
- Πλοηγείστε μακριά από οθόνες με στοιχεία FastComments