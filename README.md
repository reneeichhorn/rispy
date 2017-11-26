# Rispy \[[Documentation](https://reneeichhorn.gitbooks.io/rispy-documentation/content/)\]

## Specificiation

### 1.0.0 Objects

An object formally describes a set of data. An object can be created anytime during runtime using different approaches.

YAML notation:

```
object-name:
  type: object
  ...
```

#### 1.1.0 Initial Value

The initial value describes the data values after it is created.

YAML notation:

```
initialValue: 
  myValue: {expression}
```

#### 1.2.0 Rules

Object rules formally describe the values during any situation.  
During runtime it is garantueed that all rules apply during any situation:

* after creation
* before mutation
* after mutation

These rules allow heavy compiler optimizations and help specifiyng the object in a formal way.

YAML notation:

```
rules: 
  - ...
  - ...
```

#### 1.3.0 Namespaces

A namespace has no functionallity besides helping with naming conflicts of objects and flows.  
Each object must be inside a namespace. An object may not exist on the root level.

YAML notation:

```
namespaces: 
  - object-name:
    ...
```

### 2.0.0 Flows

#### 2.1.0 Importing flows

Flows that are defined outside the program for example core flows or 3rd party flow may be defined before usage.

YAML notation:

```
importFlows: 
  - name: my-imported-flow
    from: namespace/actual-flow-
  - ...
```

#### 2.2.0 Flow definition

To create and use a new flow it must be formally defined. The definition must contain what data it accepts and what it outputs.

YAML notation:

```
ownFlows: 
  namespace/my-own-flow
    accepts:
      - ...
    outputs:
      - ..
  ...
```

##### 2.2.1 Input and Output types

An input or output type can either be an object, a type or another stream. A stream must be formally described as well, it is not neccessary to have both an definition for accepting and outputting types \(only when being used\). When outputting a stream that emits numbers it must be described as so. When the accepting types are not defined it is assumed that it equals the outputting types \(same vice versa\). Streams are not allowed to emit or read types that are not defines as so.

YAML notation:

```
ownFlows: 
  namespace/my-own-flow
    accepts:
      - type: type-or-object-name
    outputs:
      - type: stream
        outputs:
          - ...
  ...
```

### Appendix

### 1. YAML Types

* string written as: `"This may contain Unicode characters!"`
* number written as `4361`, `-12`, `0.5` or `-0.123`



