### Maven

Προσθέστε το αποθετήριο Repsy στο POM του έργου σας:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Στη συνέχεια προσθέστε τις εξαρτήσεις που χρειάζεστε:

```xml
<dependencies>
    <!-- Πελάτης API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Βασική Βιβλιοθήκη (περιλαμβάνει SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Βιβλιοθήκη PubSub (για ζωντανά γεγονότα) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Προσθέστε το αποθετήριο Repsy στο αρχείο build.gradle σας:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Πελάτης API
    implementation "com.fastcomments:client:2.0.0"
    
    // Βασική Βιβλιοθήκη (περιλαμβάνει SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // Βιβλιοθήκη PubSub (για ζωντανά γεγονότα)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Περιεχόμενα βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τρία modules. Τον παραγόμενο πελάτη API, τη βασική βιβλιοθήκη Java που περιέχει χειροποίητες βοηθητικές λειτουργίες για να διευκολύνει την εργασία με το API, και το module `pubsub` που είναι μια βιβλιοθήκη για εγγραφή σε ροές αλλαγών.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Τεκμηρίωση Βασικής Βιβλιοθήκης, Συμπεριλαμβανομένων Παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Τεκμηρίωση Βιβλιοθήκης PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Δημόσια έναντι Ασφαλών API

Για τον πελάτη API, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi`, και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το API key σας, και η `PublicApi` περιέχει μεθόδους που μπορούν να γίνουν απευθείας από έναν περιηγητή/κινητή συσκευή/κτλ χωρίς ταυτοποίηση.

Το `ModerationApi` τροφοδοτεί τον πίνακα ελέγχου των συντονιστών. Περιέχει μεθόδους για τη διαχείριση σχολίων (λίστα, καταμέτρηση, αναζήτηση, αρχεία καταγραφής, και εξαγωγή), ενέργειες εποπτείας (αφαίρεση/επανάκτηση, σήμανση, ορισμός κατάστασης αναθεώρησης/spam/έγκρισης, ψήφοι, και επαναφορά/κλείσιμο νήματος), αποκλεισμούς (αποκλεισμός από σχολιασμό, αναίρεση αποκλεισμού, προ-συνοπτικά πριν από αποκλεισμό, κατάσταση και προτιμήσεις αποκλεισμού, και μετρήσεις αποκλεισμένων χρηστών), και σήματα & αξιοπιστία (απονομή/αφαίρεση σήματος, χειροκίνητα σήματα, λήψη/ρύθμιση παράγοντα αξιοπιστίας, και εσωτερικό προφίλ χρήστη). Κάθε μέθοδος του `ModerationApi` δέχεται παράμετρο `sso` ώστε η κλήση να μπορεί να εκτελεστεί εκ μέρους ενός συντονιστή που έχει πιστοποιηθεί μέσω SSO.