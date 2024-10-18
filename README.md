# CUMD
An easy way to make custom markdown

## The Essentials
To run cumd you need to files a style file `style.cmds` and an input `input.cmdf` and the command will generate `output.html` or if you wish to name it differently you can pass the name as an argument to the command.  
```
cumd style.cmds input.cmdf
```

The Style file includes `elements` that each contain a `key` a `nickname` `modifiers` and `html` in this format:
```
key : nickname / modifier-1 / modifier-2 {
    <HTML>{{content}}</HTML>
}
```
The `key` is the character(s) that will be used to replace with an element.
For example `>` for a blockquote or a `#` for a heading.

The `nickname` is just an optional name you can add to keep things clean

`modifiers` are ways to control the way the element acts like whether you want for the content inside to also be rendered using styling, this modifier is `recursive`

`html` is the element itself just adding `{{content}}` where you want the content (text) to be replaced.

### Let's make an example
With quotes!
I want them just to be a simple gray line to the left of the text so i'll just make quick html for that:
```
<div style="display:flex;">
       <div style="
                   width: 2px;
                   border-radius: 10%;
                   margin-right: 5px;
                   background-color: gray;"
       ></div>
   <span>{{content}}</span>
</div>
```
The normal key for a quote is `>`, I want to have quotes inside quotes of course so I'll use the `recursive` modifier.  
That will end up looking like this:
```
> : quote / recursive {
   <div style="display:flex;">
       <div style="
                   width: 2px;
                   border-radius: 10%;
                   margin-right: 5px;
                   background-color: gray;"
       ></div>
       <span>{{content}}</span>
   </div>
}
```
And when you do for you input
```
I will now write a quote
> I have writen a quote
```
Your output will be:  

---
I will now write a quote
<div style="display:flex;">
    <div style="
                width: 2px;
                border-radius: 10%;
                margin-right: 5px;
                background-color: gray;"
    ></div>
    <span> I have writen a quote</span>
</div>

---

>[!CAUTION]
  New Lines haven't been implemented completeley but there is a workaround

I made fuller example.
Check out [this](/style.cmds) style file and [this](input.cmdf) input file for this output:


## TODO LIST:
- [ ] Let `recursive` modifier go into next lines stopping at a predefined value [EX. > : quote / recursive doublenewline { ... } will take everithing as content until reaching the end condition]
- [ ] Related with ^ but implement `until x` modifier that will take content untill `x` is seen.
- [ ] stop until document ends not until an empty line [BUG] change `loop` to `for` main ln:94
- [ ] Think of a way to implement tables.
  
## Future Ideas
Ideas that I might implement but still need mor thought before placing them in the todo list
### Variables
Let there be variables inside the keys for things like lists
```_
{num}. : numbered_list {
    <div style="smth"></div>
    <span>{{num}}.</span></separator><span>{{content}}</span>
}
```
This is just a bad example of course but you get the idea  
You could also type check & stuff
```
{v: int}. : numbered_list { ... }

{v: boolean} : bool { ... }
```
or check if it matches certain conditions using rust
```
{letter: char if{letter.is_alphabetic()}}. : alphabetic_list { ... }
{number: int}. : numbered_list { ... }
```
as long as it returns a boolean value.

### CSS
This was part of the original idea that you could add `+css` if you wanted to add css to the element like>
```
> : quote / recursive {
   <div style="display:flex;">
       <div class="quote_line"></div>
       <span>{{content}}</span>
   </div>
} +css {
    .quote-line {
        width: 2px;
        border-radius: 10%;
        margin-right: 5px;
        background-color: gray;
    }
}
```
but maybe its just better to make one global css file with all the styles, and just add it as an optional argument to add to the html.