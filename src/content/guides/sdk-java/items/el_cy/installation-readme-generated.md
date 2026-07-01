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
        <version>3.0.0</version>
    </dependency>
    
    <!-- Βιβλιοθήκη Core (περιλαμβάνει SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Βιβλιοθήκη PubSub (για ζωντανά συμβάντα) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Προσθέστε το αποθετήριο Repsy στο αρχείο `build.gradle` σας:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Πελάτης API
    implementation "com.fastcomments:client:3.0.0"
    
    // Βιβλιοθήκη Core (περιλαμβάνει SSO)
    implementation "com.fastcomments:core:3.0.0"
    
    // Βιβλιοθήκη PubSub (για ζωντανά συμβάντα)
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Library Contents

Αυτή η βιβλιοθήκη περιλαμβάνει τρία modules: τον παραγόμενο πελάτη API, τη βασική βιβλιοθήκη Java που περιέχει χειρογραφή utilities για να κάνει τη δουλειά με το API πιο εύκολη, και το module `pubsub` που είναι μια βιβλιοθήκη για εγγραφή σε ροές αλλαγών.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Τεκμηρίωση Βασικής Βιβλιοθήκης, Συμπεριλαμβανομένων Παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Τεκμηρίωση Βιβλιοθήκης PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Για τον πελάτη API, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi` και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, ενώ η `PublicApi` περιέχει μεθόδους που μπορούν να κληθούν απευθείας από πρόγραμμα περιήγησης/συσκευή κινητής χωρίς πιστοποίηση.

Η `ModerationApi` παρέχει μια εκτενή σειρά ζωντανών και γρήγορων APIs διαχείρισης. Κάθε μέθοδος της `ModerationApi` δέχεται μια παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.