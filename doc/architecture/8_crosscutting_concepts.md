# Crosscutting Concepts

## Document software anomalies
Software anomalies are documented in the code with the following template:
````
// Anomaly-xxx: <Short description>
//
// <Long description>
````

Where `Anomaly-xxx` represent a unique anomaly identifier, with `xxx` as a increasing 3 digits number.
The anomaly identifier should be used to refer to an anomaly in the documentation or in the code.

When an anomaly is resolved, the comment is removed from the code base. There is no centralized list of anomalies maintained. `git grep` should be used to recover past-anomalies descriptions.