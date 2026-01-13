Προσθέστε το FastComments SDK στο αρχείο `build.gradle.kts` της εφαρμογής σας:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Βεβαιωθείτε ότι έχετε το αποθετήριο Repsy στο αρχείο `settings.gradle.kts` του έργου σας:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```