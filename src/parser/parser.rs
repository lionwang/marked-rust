#[derive(Debug)]
pub struct Config {

}

impl Config {

    pub fn new() -> Config {
        Config {

        }
    }

}

#[derive(Debug)]
struct Rule {
    newline: Regex,
    code: Regex,
    fences: Regex,
    hr: Regex,
    heading: Regex,
    blockquote: Regex,
    list: Regex,
    html: Regex,
    def: Regex,
    nptable: (),
    table: (),
    lheading: Regex,
    _paragraph: Regex,
    text: Regex,
}

trait token {

    const newline = Regex::new("").unwrap();
    

}

#[derive(Debug)]
pub struct Lexer {
    src: String,
    config: Config,
    rule: Rule,
}

use regex::Regex;

#[allow(dead_code, unused)]
impl Lexer {

    pub fn new(src: String, config: Config) -> Lexer {
        Lexer {
            src: src,
            config: config,
            rule: Rule {
                newline: Regex::new("").unwrap(),
                code: Regex::new("").unwrap(),
                fences: Regex::new("").unwrap(),
                hr: Regex::new("").unwrap(),
                heading: Regex::new("").unwrap(),
                blockquote: Regex::new("").unwrap(),
                list: Regex::new("").unwrap(),
                html: Regex::new("").unwrap(),
                def: Regex::new("").unwrap(),
                nptable: (),
                table: (),
                lheading: Regex::new("").unwrap(),
                _paragraph: Regex::new("").unwrap(),
                text: Regex::new("").unwrap(),
            }
        }
    }

    pub fn lex(&self) -> &String {
        &self.src
    }

    pub fn token(&self, top: bool) {
        let re = Regex::new(r"(?m)^ +$").unwrap();
        re.replace_all(&self.src, "");
        let mut next; 
        let mut loose;
        let mut cap;
        let mut bull;
        let mut b;
        let mut item;
        let mut listStart;
        let mut listItems;
        let mut t;
        let mut space;
        let mut i;
        let mut tag;
        let mut l;
        let mut isordered;
        let mut istask;
        let mut ischecked;
        
        while !self.src.is_empty() {
            // newline
        }


//   while (src) {
//     // newline
//     if (cap = this.rules.newline.exec(src)) {
//       src = src.substring(cap[0].length);
//       if (cap[0].length > 1) {
//         this.tokens.push({
//           type: 'space'
//         });
//       }
//     }

//     // code
//     if (cap = this.rules.code.exec(src)) {
//       var lastToken = this.tokens[this.tokens.length - 1];
//       src = src.substring(cap[0].length);
//       // An indented code block cannot interrupt a paragraph.
//       if (lastToken && lastToken.type === 'paragraph') {
//         lastToken.text += '\n' + cap[0].trimRight();
//       } else {
//         cap = cap[0].replace(/^ {4}/gm, '');
//         this.tokens.push({
//           type: 'code',
//           codeBlockStyle: 'indented',
//           text: !this.options.pedantic
//             ? rtrim(cap, '\n')
//             : cap
//         });
//       }
//       continue;
//     }

//     // fences
//     if (cap = this.rules.fences.exec(src)) {
//       src = src.substring(cap[0].length);
//       this.tokens.push({
//         type: 'code',
//         lang: cap[2] ? cap[2].trim() : cap[2],
//         text: cap[3] || ''
//       });
//       continue;
//     }

//     // heading
//     if (cap = this.rules.heading.exec(src)) {
//       src = src.substring(cap[0].length);
//       this.tokens.push({
//         type: 'heading',
//         depth: cap[1].length,
//         text: cap[2]
//       });
//       continue;
//     }

//     // table no leading pipe (gfm)
//     if (cap = this.rules.nptable.exec(src)) {
//       item = {
//         type: 'table',
//         header: splitCells(cap[1].replace(/^ *| *\| *$/g, '')),
//         align: cap[2].replace(/^ *|\| *$/g, '').split(/ *\| */),
//         cells: cap[3] ? cap[3].replace(/\n$/, '').split('\n') : []
//       };

//       if (item.header.length === item.align.length) {
//         src = src.substring(cap[0].length);

//         for (i = 0; i < item.align.length; i++) {
//           if (/^ *-+: *$/.test(item.align[i])) {
//             item.align[i] = 'right';
//           } else if (/^ *:-+: *$/.test(item.align[i])) {
//             item.align[i] = 'center';
//           } else if (/^ *:-+ *$/.test(item.align[i])) {
//             item.align[i] = 'left';
//           } else {
//             item.align[i] = null;
//           }
//         }

//         for (i = 0; i < item.cells.length; i++) {
//           item.cells[i] = splitCells(item.cells[i], item.header.length);
//         }

//         this.tokens.push(item);

//         continue;
//       }
//     }

//     // hr
//     if (cap = this.rules.hr.exec(src)) {
//       src = src.substring(cap[0].length);
//       this.tokens.push({
//         type: 'hr'
//       });
//       continue;
//     }

//     // blockquote
//     if (cap = this.rules.blockquote.exec(src)) {
//       src = src.substring(cap[0].length);

//       this.tokens.push({
//         type: 'blockquote_start'
//       });

//       cap = cap[0].replace(/^ *> ?/gm, '');

//       // Pass `top` to keep the current
//       // "toplevel" state. This is exactly
//       // how markdown.pl works.
//       this.token(cap, top);

//       this.tokens.push({
//         type: 'blockquote_end'
//       });

//       continue;
//     }

//     // list
//     if (cap = this.rules.list.exec(src)) {
//       src = src.substring(cap[0].length);
//       bull = cap[2];
//       isordered = bull.length > 1;

//       listStart = {
//         type: 'list_start',
//         ordered: isordered,
//         start: isordered ? +bull : '',
//         loose: false
//       };

//       this.tokens.push(listStart);

//       // Get each top-level item.
//       cap = cap[0].match(this.rules.item);

//       listItems = [];
//       next = false;
//       l = cap.length;
//       i = 0;

//       for (; i < l; i++) {
//         item = cap[i];

//         // Remove the list item's bullet
//         // so it is seen as the next token.
//         space = item.length;
//         item = item.replace(/^ *([*+-]|\d+\.) */, '');

//         // Outdent whatever the
//         // list item contains. Hacky.
//         if (~item.indexOf('\n ')) {
//           space -= item.length;
//           item = !this.options.pedantic
//             ? item.replace(new RegExp('^ {1,' + space + '}', 'gm'), '')
//             : item.replace(/^ {1,4}/gm, '');
//         }

//         // Determine whether the next list item belongs here.
//         // Backpedal if it does not belong in this list.
//         if (i !== l - 1) {
//           b = block.bullet.exec(cap[i + 1])[0];
//           if (bull.length > 1 ? b.length === 1
//             : (b.length > 1 || (this.options.smartLists && b !== bull))) {
//             src = cap.slice(i + 1).join('\n') + src;
//             i = l - 1;
//           }
//         }

//         // Determine whether item is loose or not.
//         // Use: /(^|\n)(?! )[^\n]+\n\n(?!\s*$)/
//         // for discount behavior.
//         loose = next || /\n\n(?!\s*$)/.test(item);
//         if (i !== l - 1) {
//           next = item.charAt(item.length - 1) === '\n';
//           if (!loose) loose = next;
//         }

//         if (loose) {
//           listStart.loose = true;
//         }

//         // Check for task list items
//         istask = /^\[[ xX]\] /.test(item);
//         ischecked = undefined;
//         if (istask) {
//           ischecked = item[1] !== ' ';
//           item = item.replace(/^\[[ xX]\] +/, '');
//         }

//         t = {
//           type: 'list_item_start',
//           task: istask,
//           checked: ischecked,
//           loose: loose
//         };

//         listItems.push(t);
//         this.tokens.push(t);

//         // Recurse.
//         this.token(item, false);

//         this.tokens.push({
//           type: 'list_item_end'
//         });
//       }

//       if (listStart.loose) {
//         l = listItems.length;
//         i = 0;
//         for (; i < l; i++) {
//           listItems[i].loose = true;
//         }
//       }

//       this.tokens.push({
//         type: 'list_end'
//       });

//       continue;
//     }

//     // html
//     if (cap = this.rules.html.exec(src)) {
//       src = src.substring(cap[0].length);
//       this.tokens.push({
//         type: this.options.sanitize
//           ? 'paragraph'
//           : 'html',
//         pre: !this.options.sanitizer
//           && (cap[1] === 'pre' || cap[1] === 'script' || cap[1] === 'style'),
//         text: this.options.sanitize ? (this.options.sanitizer ? this.options.sanitizer(cap[0]) : escape(cap[0])) : cap[0]
//       });
//       continue;
//     }

//     // def
//     if (top && (cap = this.rules.def.exec(src))) {
//       src = src.substring(cap[0].length);
//       if (cap[3]) cap[3] = cap[3].substring(1, cap[3].length - 1);
//       tag = cap[1].toLowerCase().replace(/\s+/g, ' ');
//       if (!this.tokens.links[tag]) {
//         this.tokens.links[tag] = {
//           href: cap[2],
//           title: cap[3]
//         };
//       }
//       continue;
//     }

//     // table (gfm)
//     if (cap = this.rules.table.exec(src)) {
//       item = {
//         type: 'table',
//         header: splitCells(cap[1].replace(/^ *| *\| *$/g, '')),
//         align: cap[2].replace(/^ *|\| *$/g, '').split(/ *\| */),
//         cells: cap[3] ? cap[3].replace(/\n$/, '').split('\n') : []
//       };

//       if (item.header.length === item.align.length) {
//         src = src.substring(cap[0].length);

//         for (i = 0; i < item.align.length; i++) {
//           if (/^ *-+: *$/.test(item.align[i])) {
//             item.align[i] = 'right';
//           } else if (/^ *:-+: *$/.test(item.align[i])) {
//             item.align[i] = 'center';
//           } else if (/^ *:-+ *$/.test(item.align[i])) {
//             item.align[i] = 'left';
//           } else {
//             item.align[i] = null;
//           }
//         }

//         for (i = 0; i < item.cells.length; i++) {
//           item.cells[i] = splitCells(
//             item.cells[i].replace(/^ *\| *| *\| *$/g, ''),
//             item.header.length);
//         }

//         this.tokens.push(item);

//         continue;
//       }
//     }

//     // lheading
//     if (cap = this.rules.lheading.exec(src)) {
//       src = src.substring(cap[0].length);
//       this.tokens.push({
//         type: 'heading',
//         depth: cap[2].charAt(0) === '=' ? 1 : 2,
//         text: cap[1]
//       });
//       continue;
//     }

//     // top-level paragraph
//     if (top && (cap = this.rules.paragraph.exec(src))) {
//       src = src.substring(cap[0].length);
//       this.tokens.push({
//         type: 'paragraph',
//         text: cap[1].charAt(cap[1].length - 1) === '\n'
//           ? cap[1].slice(0, -1)
//           : cap[1]
//       });
//       continue;
//     }

//     // text
//     if (cap = this.rules.text.exec(src)) {
//       // Top-level should never reach here.
//       src = src.substring(cap[0].length);
//       this.tokens.push({
//         type: 'text',
//         text: cap[0]
//       });
//       continue;
//     }

//     if (src) {
//       throw new Error('Infinite loop on byte: ' + src.charCodeAt(0));
//     }
//   }

//   return this.tokens;

//     }
    }

}

fn newline(src: &String) {

}

#[derive(Debug)]
pub struct Parser {

}

impl Parser {
    
}

