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

Στη συνέχεια, προσθέστε τις εξαρτήσεις που χρειάζεστε:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
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
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Περιεχόμενα βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τρεις μονάδες. Ο παραγόμενος client API, η βασική βιβλιοθήκη Java η οποία περιέχει χειροποίητα βοηθητικά εργαλεία για να διευκολύνει την εργασία με το API, και το module `pubsub` που είναι μια βιβλιοθήκη για εγγραφή σε ροές αλλαγών.

- [Τεκμηρίωση βιβλιοθήκης API Πελάτη](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Τεκμηρίωση βασικής βιβλιοθήκης, συμπεριλαμβανομένων παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Τεκμηρίωση βιβλιοθήκης PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Δημόσια έναντι Προστατευμένων API

Για τον πελάτη API, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi`, και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, και η `PublicApi` περιέχει μεθόδους που μπορούν να γίνουν απευθείας από ένα πρόγραμμα περιήγησης/κινητή συσκευή/κ.λπ. χωρίς πιστοποίηση.

Η `ModerationApi` τροφοδοτεί τον πίνακα ελέγχου (dashboard) του διαχειριστή. Περιέχει μεθόδους για την επιτήρηση σχολίων (λίστα, καταμέτρηση, αναζήτηση, αρχεία καταγραφής και εξαγωγή), ενέργειες επιτήρησης (αφαίρεση/επαναφορά, σήμανση, ορισμός κατάστασης αναθεώρησης/spam/έγκρισης, ψήφοι, και επαναφορά/κλείσιμο νήματος), αποκλεισμούς (απαγόρευση σχολιασμού, άρση αποκλεισμού, προ-αποκλειστικές περιλήψεις, κατάσταση και προτιμήσεις αποκλεισμού, και αριθμοί αποκλεισμένων χρηστών), και διακριτικά & εμπιστοσύνη (απονομή/αφαίρεση διακριτικού, χειροκίνητα διακριτικά, λήψη/ρύθμιση παράγοντα εμπιστοσύνης, και εσωτερικό προφίλ χρήστη). Κάθε μέθοδος της `ModerationApi` δέχεται παράμετρο `sso`, ώστε η κλήση να μπορεί να εκτελεστεί εκ μέρους ενός διαχειριστή που έχει πιστοποιηθεί μέσω SSO.