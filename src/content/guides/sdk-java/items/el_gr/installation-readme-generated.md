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
        <version>3.1.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
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
    implementation "com.fastcomments:client:3.1.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Library Contents

Αυτή η βιβλιοθήκη περιέχει τρία modules. Το παραγόμενο API client, τη βασική βιβλιοθήκη Java που περιέχει χειρογραφούμενα βοηθητικά εργαλεία για να διευκολύνει τη χρήση του API, και το module `pubsub` που είναι μια βιβλιοθήκη για την εγγραφή σε ροές αλλαγών.

- [Τεκμηρίωση Βιβλιοθήκης API Client](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Τεκμηρίωση Βασικής Βιβλιοθήκης, Συμπεριλαμβανομένων Παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Τεκμηρίωση Βιβλιοθήκης PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Για το API client, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi` και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, ενώ η `PublicApi` περιέχει μεθόδους που μπορούν να κληθούν απευθείας από έναν περιηγητή/συσκευή κινητής τηλεφωνίας/κτλ χωρίς έλεγχο ταυτότητας.

Η `ModerationApi` παρέχει μια εκτενή σειρά από ζωντανά και γρήγορα APIs διαχείρισης. Κάθε μέθοδος της `ModerationApi` δέχεται μια παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.