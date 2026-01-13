### Αποτροπή Διαρροών Μνήμης

Για να αποτρέψετε διαρροές μνήμης όταν χρησιμοποιείτε προβολές FastComments σε Activities ή Fragments, καλέστε πάντα την `cleanup()` όταν η προβολή δεν χρειάζεται πλέον:

#### Σε Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // Καθαρισμός προβολών FastComments για αποφυγή διαρροών μνήμης
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
    // Καθαρισμός προβολών FastComments όταν η προβολή του fragment καταστραφεί
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // Επιπρόσθετος καθαρισμός όταν το fragment καταστρέφεται
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Κατά την εναλλαγή Fragments:
```java
// Πριν αντικαταστήσετε ή αφαιρέσετε ένα fragment που περιέχει προβολές FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// Then proceed with fragment transaction
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**Σημαντικό**: Καλέστε πάντα τις μεθόδους `cleanup()` για να αποτρέψετε διαρροές μνήμης, ειδικά όταν:
- Activities καταστρέφονται
- Οι προβολές των Fragment καταστρέφονται
- Εναλλαγή μεταξύ fragments
- Πλοήγηση μακριά από οθόνες με στοιχεία FastComments