# About `jxpand`

`jxpand` is a tool for expanding JSON values from the command line.

The expanded JSON includes additional annotations that make it easier to use with templating engines such as [mustache][mustache].

The API is still in development and breaking changes may occur until the 1.0 release.

## Annotations

JSON arrays are expanded to include additional annotations relevant to the items in the array.

For example, the array:
    
```json
[ "foo", "bar", "baz" ]
```

is expanded to:

```json
{
    "values": [
        {
            "value": "foo",
            "first": true,
            "last": false,
            "index": 0
        },
        {
            "value": "bar",
            "first": false,
            "last": false,
            "index": 1
        },
        {
            "value": "baz",
            "first": false,
            "last": true,
            "index": 2
        }
    ],
    "count": 3
}
```

- The `count` annotation is added by substituting the array with an object containing the `count` key and the original array in the `values` key.
- The `first` annotation is `true` for the first item in the array.
- The `last` annotation is `true` for the last item in the array.
- The `index` annotation is added to every item in the array with the index of the item.

### Merge mode

Instead of wrapping objects in an array, you can merge the annotations into existing objects. Each annotation is added with a prefix.

For example, the array:

```json
[
    {
        "foo": "bar"
    },
    {
        "foo": "baz"
    }
]
```

is expanded to:

```json
{
    "values": [
        {
            "foo": "bar",
            "_first": true,
            "_last": false,
            "_index": 0
        },
        {
            "foo": "baz",
            "_first": false,
            "_last": true,
            "_index": 1
        }
    ],
    "count": 2
}
```

### Disabling annotations

All annotations are enabled by default, and disabling an annotation will adjust the output accordingly. If the count annotation is enabled, arrays will no longer be wrapped.

Currently, disabling all item-level annotations will still result in each item being wrapped.

## Command line interface

The `jxpand` tool can be used to expand JSON values from the command line. Its usage can be seen by running:

```bash
$ jxpand --help
```

It supports:

- Disabling individual annotations
- Adjusting the prefix (for use when merging annotations)
- Merging annotations into existing objects
- Pretty-printing the output

## Roadmap

- [ ] Prevent wrapping of items when all item-level annotations are disabled
- [ ] Add "joiners" to support adding an arbitrary value accessible on all but the last element. For example, a comma could be added to the end of each item in an array, except the last item.
- [ ] Cycle annotations which can be used to cycle through a list of values. For example, a list of colors could be cycled through when rendering a template.

Submit an issue if there is something you would like to see.

[mustache]: https://mustache.github.io/