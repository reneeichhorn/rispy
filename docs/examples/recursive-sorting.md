# Learn by examples: Recursive Sorting

In this example we're going to show you how to implement very simple sorting algorithm. This example is not meant to be a fast implementation instead it shows the simplest implementation.

## Lowest value

Think very simple, what is supposed to be the first result? Well it has to be the lowest value of an "array" \(or the biggest for descending sorting\)

To find out what the lowest value is we're going to create a new object that will take care about

```yaml
namespace:
  util:
    lowest-number:
      type: object
      initialValue:
        lowestValue: 0
      rules:
        - targets: lowestValue
          type: private
        - targets: lowestValue
          type: condition
          condition:
            type: any
            of: 
              type: stream
              expression: stream
            must:
              type: leq
              left: 
                type: accesor
                expression: lowestValue
              right:
                type: anyValue
```

We can ignore the initialState for now.. if you think about it there is none actually. Next the rules:

* lowestValue is private: we don't want that anyone outside this object can see this.
* any value in the stream will be lower or equal the lowestValue: this might a bit trick, but we're just formally defining that lowestValue is indeed the lowestValue within a stream. Note that rules must be valid to all given times. We'll talk about this rule later again.

## Importing core streams

```
importFlows:
  - name: application-start
    from: core/application-start
  - name: stdout
    from: core/terminal/stdout
```

To use any core stream or third party stream we need to first import it:

* importing and using core/application-start is like declaring a main function in any other programing langauge
* core/terminal/stdout: we're going to use stdout to output our sorted results so that we can actually verify that our progam is working correctly

## Declaring our own streams

```
ownFlows:
  util/lowest-number/lowest:
    accepts:
      - type: stream
        outputs:
          - type: number
    outputs:
      - type: number

  util/lowest-number/sorting:
    accepts:
      - type: stream
        outputs:
          - type: number
    outputs:
      - type: number
```

Just like importing existing streams we need to specify the look of the streams that we're going to use:

* util/lowest-number/lowest: With this naming we are automatically putting the stream into objects namespace. It's not required but in raw yaml it is easier to read. This stream will accept a stream that outputs numbers. This is very similar to an array of numbers in any other language. It also can output numbers \(which will be the lowest number of the incoming stream\).
* util/lowest/number/sorting: The definition here is exactly the same as our definition of /lowest. You might wonder why it outputs number. Earlier we said that a stream of numbers are similar to an array of numbers so why does it only output  numbers instead of a stream of numbers. It is never a requirement to have one output for one input. Meaning that if we put one stream into it it might also respond with multiple numbers. A stream of numbers can also just that it's a group of numbers. We'll talk about it again when we are implementing this stream.

## The application-start / main function

```
application-start:
  outputs:
    - type: intoStream
      stream: util/lowest-number/sorting
      converter:
        type: constant
        constantType: stream
        value:
          - type: number
            value: 4 
          - type: number
            value: 1
          - type: number
            value: 2
          - type: number
            value: 8
          - type: number
            value: 3
          - type: number
            value: 6
          - type: number
            value: 1
      links:
        - to: util/lowest-number/sorting
          outputs:
            - type: intoStream
              stream: stdout
              converter:
                type: core/converter/number/toString
```

Let's pretend for now that the the sorting stream is already working as we intended it to be. Therefore let's start with the application start. Whenever the application-start emits something we using the intoStream operation. For the application-start this will of course only happen once. The intoStream operation simply puts what ever was emitted in the parent stream into another stream. It this case it puts whatever application-start emits into the util/lowest-number/sorting stream. Of course the output of the application-start is nothing that our sorting stream could take care of. To change what data goes into the sorting stream we can define a converter. There are multiple types of converter for now lets only care about the constant converter.

The constant converter is actually very special, to be fair it is not converting anything actually. It doesn't even care about what the stream gives it. No matter what input it gets it will "convert" it into an constant value. In this case a stream that emits: 4, 1, 2, 8, 3, 6, 1 \(our test numbers that we want to sort\)

The next thing we're going to talk about are links. The intoStream operation supports the so called linking feature.

### Linking

You already noticed that a stream can take value and it will output values. Putting something into stream might trigger many things. With a link we can actually follow these "triggers". For example you'd have a logging stream. The way you implemented it, it will whenever a "log" comes in it will add some formatting and then print it into stdout. So whatever you put into it, at some point will be redirected to the stdout \(converted to fit into the std stream definition\)

A link just allows you to "follow" what you just put into a stream until it reaches a certain destination. The simplest use case for this is if you want to get a "return" value of something you put into a stream. In our case we want to "follow" or unsorted numbers to the point they are sorted and then put them into stdout using the core/converter/number/toString converter \(stdout only accepts strings\)

## The lowest stream

```
util/lowest-number/lowest:
  outputs:
    - type: mergeIntoSubstream
      object: util/lowest-number
      outputs:
        - type: condition
          condition:
            type: lt
            left:
              type: accesor
              expression: lowestValue
            right:
              type: stream
              expression: stream
            reactions:
              - type: mutator
                expression: lowestValue
                value: 
                  type: stream
                  expression: stream
        - type: ends
          outputs:
            - type: intoStreamOutput
              stream: util/lowest-number
              converter:
                type: accesor
                expression: lowestValue
```

Now it gets a little bit more complex.. Let's introduce 3 new operations:

### mergeIntoSubstream

The simplest way to describe a substream is an anonymous function. It is a stream that doesn't have any name nor will be visible to the outside. Instead of just intoSubstream we're using mergeIntoSubstream though. Remember that we're putting a stream of numbers into the sorting stream. Since we want to look at all the individual numbers we need to merge them. Merging just means  putting the content of the stream into a stream. So instead of putting whatever comes into lowest to the substream we're putting whatever is emitted from whatever comes into stream.

One new option you might have noticed is the object option. This binds a object to the stream, meaning that whenever this stream is created it will assign an object to it and initialize it.

### condition

The condition operation is a very simple one, this is just normal branching if you want to something if a certain condition applies the you can use the condition operation. You might also have noticed the reactions option. This one is actually available on all operations. A reaction allows you to modify the assigned object. In this case it mutates the value to the current number if it is smaller than the current value.

### ends

Every stream ends at some point, either when the program exits or when the stream is done with whatever it was doing. In our case the stream is done when all of our numbers have been processed.

### intoStreamOutput

This one is very similar to the intoStream operation that we already know. In RISPY a stream can receive values and it can output values. intoStream puts something into a stream. With intoStreamOutput we tell a stream to output something. Note that the target stream can only be the parent stream.

## The sorting stream

```
util/lowest-number/sorting:
  outputs:
    - type: intoStream
      stream: util/lowest-number/lowest
      links:
        - to: util/lowest-number/lowest
          outputs:
            - type: intoStreamOutput
              stream: util/lowest-number/sorting
              converter:
                type: none
          delink:
            outputs:
              - type: ends
                outputs:
                - type: intoStream
                  stream: util/lowest-number/sorting
                  converter:
                    type: none
```

Now that we have a stream that gives us the lowest number of a stream we can easily write a recursive stream that sorts the whole stream. This works the same as any "normal" recursive function. One new thing we need to introduce first though is the delink.

### Delinking

We already know what a link is but what is delink? It's quite simple, we already know that a link is following the input til it reaches a certain destination. What happens with the input that didn't reach this destination though? There we go and this exactly where a delink comes in, whenever the desired destination is not reached we can use the delink to follow inputs that didn't reach the destination.

In our sorting example its very simple, we put multiple numbers into the lowest stream. The lowest number will reach the output of the lowest stream the rest won't. therefore we just put the lowest number into our own output stream and then we put the remaining numbers again into the sorting stream.

## Performance

Lets have quick look at the performance of the stream we just created. The good thing is the complexity is always the same no matter how badly it is sorted.

The complexity is:

$$O(n) = 1 + 2 +3 + ... + (n-2) + (n-1) + n$$

written a little bit simpler:

$$O(n) = (n(n+1))/2$$

