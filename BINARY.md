# Straw Proposal Binary Format for Wasm Web IDL Bindings

This document describes the straw proposal binary format that this crate
currently supports.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [The Web IDL Bindings Custom Section](#the-web-idl-bindings-custom-section)
- [Subsections](#subsections)
- [The Web IDL Type Subsection](#the-web-idl-type-subsection)
  - [Web IDL Functions](#web-idl-functions)
  - [Web IDL Dictionaries](#web-idl-dictionaries)
  - [Web IDL Enumerations](#web-idl-enumerations)
  - [Web IDL Unions](#web-idl-unions)
- [References to Web IDL Types](#references-to-web-idl-types)
- [The Function Binding Subsection](#the-function-binding-subsection)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## The Web IDL Bindings Custom Section

**Custom section name:** `webidl-bindings`.

## Subsections

The data of a Web IDL Bindings custom section contains a sequence of
subsections. Each subsection consists of:

* a one-byte *id*,
* a `u32` *size* of its contents, in bytes,
* the subsection's actual *contents*, whose structure is determined by its
  subsection *id*.

```
webidl_bindings_sec ::= section[0](webidl_bindings_data)

webidl_bindings_data ::= n:name                 (if name = "webidl-bindings")
                         webidl_type_subsec?
                         bindings_subsec

webidl_bindings_subsection[N](B) ::= N:byte size:u32 B   (if size = |B|)
```

The following subsection ids are used:

| Id | Subsection              |
|:--:|:------------------------|
| 0  | Web IDL Type Subsection |
| 1  | Bindings Subsection     |

## The Web IDL Type Subsection

The Web IDL Type Subsection is a sequence of `webidl_type` definitions:

```
webidl_type_subsec ::= webidl_bindings_subsection[0](vec(webidl_type))
```

A `webidl_type` is a function, dictionary, enumeration, or union:

```
webidl_type ::= 0x0 webidl_function
            ::= 0x1 webidl_dictionary
            ::= 0x2 webidl_enumeration
            ::= 0x3 webidl_union
```

### Web IDL Functions

Functions are encoded as their function kind, which is one of

* static (0x0)
* method and receiver type (0x1)
* constructor (0x2)

followed by a sequence of references to their parameter types, and an optional
reference to its result type:

```
webidl_function ::= webidl_function_kind
                    vec(webidl_function_param)
                    webidl_function_result

webidl_function_kind ::= 0x0                       # static
                     ::= 0x1 webidl_type_reference # method
                     ::= 0x2                       # constructor

webidl_function_param ::= webidl_type_reference

webidl_function_result ::= 0x0
                       ::= 0x1 webidl_type_reference
```

### Web IDL Dictionaries

Dictionaries are encodes as a `vec` of pairs of the dictionary field's UTF-8
name string and a reference to the field's value:

```
webidl_dictionary ::= vec(webidl_dictionary_field)

webidl_dictionary_field ::= name webidl_type_reference
```

### Web IDL Enumerations

Enumerations are encoded as a `vec` of their values' UTF-8 name strings.

```
webidl_enumeration ::= vec(name)
```

### Web IDL Unions

Unions are encoded as a `vec` of references to their member types:

```
webidl_union ::= vec(webidl_type_reference)
```

## References to Web IDL Types

References to Web IDL types appear in both the Web IDL Type Subsection and in
the Web IDL Function Binding Subsection.

References to Web IDL types are encoded as `i32`s and come in two forms:

1. References to compound Web IDL types are `>= 0` and are indices referencing
   the `i`th compound type defined in the Web IDL Type Subsection.
2. Primitive Web IDL values are encoded as negative numbers, and each primitive
   type is special cased.

```
webidl_type_reference ::= i:i32    (if i >= 0)     => index into Web IDL Type Subsection
                      ::= i:i32    (if i == -1)    => TODO(web idl primitive types...)
                      ::= i:i32    (if i == -2)    => TODO(web idl primitive types...)
                      ::= i:i32    (if i == -3)    => TODO(web idl primitive types...)
                      # etc...
```

## The Function Binding Subsection

TODO
