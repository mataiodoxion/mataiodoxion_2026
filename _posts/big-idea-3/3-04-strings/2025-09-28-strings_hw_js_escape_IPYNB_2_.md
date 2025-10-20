---
title: 3.04 Strings JavaScript Homework
description: Quick practice with strings in JavaScript.
permalink: /csp/big-idea-3/strings/p4/hw-js
author_profile: False
hidden: True
---

```python
# I don't want to install anything on my system without containerization.
# I'll add my answers as comments

# Interactive Free-Response Quiz (JavaScript-only)
from IPython.display import display, HTML, Javascript
import ipywidgets as widgets

print("🎯 JavaScript String Basics - Free-Response Quiz")
print("Type your answers below and click Check Answers.\n")

# Question 1
q1_label = widgets.HTML("<b>1. In JavaScript, which property is used to get the length of a string?</b><br><i>Answer with the property name (e.g., length or string.length)</i>")
q1_input = widgets.Text(placeholder="e.g., length or string.length")
# Answer: length

# Question 2
q2_label = widgets.HTML("<b>2. Which expression can concatenate two strings?</b><br><i>Type the expression or operator (e.g., \"Hello\" + \"World\" or +)</i>")
q2_input = widgets.Text(placeholder='e.g., "Hello" + "World" or +')
# Answer: "Hello" + "World"

# Question 3
q3_label = widgets.HTML("<b>3. What does 'JavaScript'.substring(0, 4) return?</b><br><i>Type the exact substring</i>")
q3_input = widgets.Text(placeholder='e.g., Java')
# Answer: Java

# Question 4
q4_label = widgets.HTML("<b>4. Which method converts a string to uppercase?</b><br><i>Type the method name</i>")
q4_input = widgets.Text(placeholder='e.g., toUpperCase()')
# Answer: toUpperCase()

# True/False Questions
q5_label = widgets.HTML("<b>5. String indexes start at 1 in JavaScript</b><br><i>Answer True or False</i>")
q5_input = widgets.Text(placeholder='True/False')
# Answer: False

q6_label = widgets.HTML("<b>6. 'hello'.length returns 5</b><br><i>Answer True or False</i>")
q6_input = widgets.Text(placeholder='True/False')
# Answer: True

# Submit Button
submit_btn = widgets.Button(
    description="Check Answers",
    button_style='success',
    icon='check'
)

output = widgets.Output()

# Helper functions for tolerant checking

def clean_token(s: str) -> str:
    s = s.strip().lower()
    s = s.strip('`')
    if s.endswith('()'):
        s = s[:-2]
    return s


def strip_quotes_spaces(s: str) -> str:
    return s.replace('"', '').replace("'", '').replace(' ', '').strip().lower()


def parse_bool_like(s: str):
    v = s.strip().lower()
    if v in {'true', 't', 'yes', 'y', '1'}:
        return True
    if v in {'false', 'f', 'no', 'n', '0'}:
        return False
    return None


def check_answers(btn):
    with output:
        output.clear_output()
        score = 0
        total = 6

        print("📊 QUIZ RESULTS")
        print("=" * 40)

        # Q1: length property
        a1 = clean_token(q1_input.value)
        if a1 == 'length' or a1.endswith('.length'):
            score += 1
            print("✅ Q1: CORRECT")
        else:
            print("❌ Q1: INCORRECT - Expected: length (property)")

        # Q2: concatenation using +
        raw2 = q2_input.value
        a2_clean = strip_quotes_spaces(raw2)
        if ('+' in raw2) or ('plus' in raw2.lower()) or (a2_clean == 'hello+world'):
            score += 1
            print("✅ Q2: CORRECT")
        else:
            print("❌ Q2: INCORRECT - Example: \"Hello\" + \"World\" (use +)")

        # Q3: substring(0,4) -> "Java"
        a3 = q3_input.value.strip().strip('"').strip("'")
        if a3.lower() == 'java':
            score += 1
            print("✅ Q3: CORRECT")
        else:
            print("❌ Q3: INCORRECT - Correct: Java")

        # Q4: toUpperCase()
        a4 = clean_token(q4_input.value).replace('_', '')
        if a4 == 'touppercase':
            score += 1
            print("✅ Q4: CORRECT")
        else:
            print("❌ Q4: INCORRECT - Correct: toUpperCase()")

        # Q5: False (indexes start at 0)
        a5 = parse_bool_like(q5_input.value)
        if a5 is None:
            print("⚠️ Q5: Could not parse. Please answer True/False.")
        elif a5 is False:
            score += 1
            print("✅ Q5: CORRECT")
        else:
            print("❌ Q5: INCORRECT - Correct: False (indexes start at 0)")

        # Q6: True ('hello'.length == 5)
        a6 = parse_bool_like(q6_input.value)
        if a6 is None:
            print("⚠️ Q6: Could not parse. Please answer True/False.")
        elif a6 is True:
            score += 1
            print("✅ Q6: CORRECT")
        else:
            print("❌ Q6: INCORRECT - Correct: True")

        print("=" * 40)
        print(f"🎯 Your Score: {score}/{total}")
        print(f"📈 Percentage: {score/total*100:.1f}%")

        # Persist a completion flag to localStorage (optional)
        try:
            display(Javascript("""localStorage.setItem('csp34_hw_quiz_tried','true');"""))
        except Exception:
            pass


submit_btn.on_click(check_answers)

# Display all questions and inputs
display(
    q1_label, q1_input,
    q2_label, q2_input,
    q3_label, q3_input,
    q4_label, q4_input,
    q5_label, q5_input,
    q6_label, q6_input,
    submit_btn, output,
)
```

## Part C: JavaScript Password Strength Practice

Here you will make a password strength analyzer in JavaScript. Complete the program so it checks:

- Password length ≥ 8 characters
- Contains uppercase letters
- Contains lowercase letters  
- Contains numbers
- Contains special characters (!@#$%^&*)

Then calculate a strength score and display appropriate feedback.

Try to fix the code to set your own password level.


```javascript
%%javascript

console.log("=== Password Strength Analyzer ===");

const password = "MyPassword123!";

const passwordLength = password.length;
const isLongEnough = passwordLength >= 14;
const hasUppercase = /[A-Z]/.test(password);
const hasLowercase = /[a-z]/.test(password);
const hasNumbers = /\d/.test(password);
const hasSpecial = /[!@#$%^&*]/.test(password);

let strengthScore = 0;
if (isLongEnough) strengthScore++;
if (hasUppercase) strengthScore++;
if (hasLowercase) strengthScore++;
if (hasNumbers) strengthScore++;
if (hasSpecial) strengthScore++;

let strengthRating = "";
if (strengthScore <= 2) strengthRating = "Weak";
else if (strengthScore <= 3) strengthRating = "Medium";
else strengthRating = "Strong";

const suggestions = [];
if (!isLongEnough) suggestions.push("Make password longer (at least 8 characters)");
if (!hasUppercase) suggestions.push("Add uppercase letters");
if (!hasLowercase) suggestions.push("Add lowercase letters");
if (!hasNumbers) suggestions.push("Add numbers");
if (!hasSpecial) suggestions.push("Add special characters: !@#$%^&*");


let output = "=== Password Strength Analyzer ===\n\n";
output += "Password: " + password + "\n";
output += "Length: " + passwordLength + " characters\n";
output += "Strength Score: " + strengthScore + "/5 - " + strengthRating + "\n\n";
output += "Requirements met:\n";
output += "  ✓ Length >= 8: " + isLongEnough + "\n";
output += "  ✓ Has uppercase: " + hasUppercase + "\n";
output += "  ✓ Has lowercase: " + hasLowercase + "\n";
output += "  ✓ Has numbers: " + hasNumbers + "\n";
output += "  ✓ Has special chars: " + hasSpecial + "\n\n";

if (suggestions.length > 0) {
    output += "Suggestions for improvement:\n";
    for (const s of suggestions) {
        output += "  • " + s + "\n";
        console.log(output);
    }
} else {
    output += "🎉 Excellent! Your password meets all security requirements!\n";
    console.log(output);
}
```


    <IPython.core.display.Javascript object>

