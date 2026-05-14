Una vez que FastComments está registrado en tu LMS, los instructores lo añaden a los cursos de la misma manera que añaden cualquier otra herramienta externa LTI.

#### D2L Brightspace

En el área de contenido de un curso:

1. Haz clic en **Add Existing Activities** > **External Learning Tools**.
2. Elige **FastComments** de la lista.
3. La herramienta aparece como un tema en el área de contenido. Ábrela una vez para inicializar el hilo de comentarios para ese recurso.

#### Moodle

En un curso:

1. Activa **Edit mode**.
2. En la sección donde quieras comentarios, haz clic en **Add an activity or resource**.
3. Elige **FastComments** en el selector de actividades.
4. Guarda. Los estudiantes ven el hilo de comentarios incrustado en la sección.

#### Blackboard Learn

En un curso:

1. Navega a un Content Area.
2. Haz clic en **Build Content** > **FastComments** (bajo "Learning Tools").
3. Configura un nombre y envía.

#### Sakai

Los mantenedores del sitio añaden la herramienta a través de **Site Info** > **Manage Tools** > desplázate hasta **External Tools** > selecciona **FastComments** > **Continue**.

#### How Threads Are Scoped

FastComments crea un hilo de comentarios por separado por **(LMS instance, course, resource link)**. Eso significa:

- Dos cursos diferentes en el mismo LMS obtienen hilos separados, incluso si usan el mismo nombre de herramienta.
- La misma herramienta FastComments utilizada en dos lugares dentro de un curso crea dos hilos.
- Dos instalaciones diferentes de Brightspace bajo la misma cuenta de FastComments obtienen hilos distintos: el nombre de host del LMS forma parte del identificador del hilo.

#### SSO

Los estudiantes no ven una pantalla de inicio de sesión. El LMS envía su identidad (nombre, correo electrónico, avatar, rol) a FastComments vía el lanzamiento LTI, y FastComments los autentica automáticamente. Sus comentarios se atribuyen a su cuenta del LMS.

Los usuarios con los roles del LMS **Instructor** o **Administrator** se asignan automáticamente a roles de moderador/administrador de FastComments para el hilo.