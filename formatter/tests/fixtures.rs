use std::fs;
use std::path::Path;

pub struct TestCase {
    pub name: &'static str,
    pub input: &'static str,
    pub expected: &'static str,
    pub description: &'static str,
}

pub const BASIC_JSON5_TESTS: &[TestCase] = &[
    TestCase {
        name: "simple_object",
        input: r#"{"name":"test","value":123}"#,
        expected: r#"{
  "name": "test",
  "value": 123
}"#,
        description: "Format a simple JSON5 object with proper indentation",
    },
    TestCase {
        name: "nested_object",
        input: r#"{"user":{"name":"John","details":{"age":30,"city":"NYC"}}}"#,
        expected: r#"{
  "user": {
    "name": "John",
    "details": {
      "age": 30,
      "city": "NYC"
    }
  }
}"#,
        description: "Format nested JSON5 objects",
    },
    TestCase {
        name: "simple_array",
        input: r#"[1,2,3,4,5]"#,
        expected: r#"[1, 2, 3, 4, 5]"#,
        description: "Format a simple array with proper spacing",
    },
    TestCase {
        name: "long_array",
        input: r#"[1,2,3,4,5,6,7,8,9,10]"#,
        expected: r#"[
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10
]"#,
        description: "Format a long array with multiline layout",
    },
];

pub const MUSTACHE_TESTS: &[TestCase] = &[
    TestCase {
        name: "simple_interpolation",
        input: r#"{{name}}"#,
        expected: r#"{{name}}"#,
        description: "Format simple mustache interpolation",
    },
    TestCase {
        name: "section_basic",
        input: r#"{{#users}}
{{name}}
{{/users}}"#,
        expected: r#"{{#users}}
  {{name}}
{{/users}}"#,
        description: "Format basic mustache section with proper indentation",
    },
    TestCase {
        name: "nested_sections",
        input: r#"{{#users}}
{{#active}}
{{name}} is active
{{/active}}
{{/users}}"#,
        expected: r#"{{#users}}
  {{#active}}
    {{name}} is active
  {{/active}}
{{/users}}"#,
        description: "Format nested mustache sections",
    },
    TestCase {
        name: "inverted_section",
        input: r#"{{^users}}
No users found
{{/users}}"#,
        expected: r#"{{^users}}
  No users found
{{/users}}"#,
        description: "Format inverted mustache section",
    },
];

pub const MIXED_CONTENT_TESTS: &[TestCase] = &[
    TestCase {
        name: "json_with_mustache_simple",
        input: r#"{
"users": [
{{#each users}}
{"name": "{{name}}", "email": "{{email}}"}{{#unless @last}},{{/unless}}
{{/each}}
]
}"#,
        expected: r#"{
  "users": [
    {{#each users}}
    {
      "name": "{{name}}",
      "email": "{{email}}"
    }{{#unless @last}},{{/unless}}
    {{/each}}
  ]
}"#,
        description: "Format JSON5 object containing mustache templates",
    },
    TestCase {
        name: "conditional_json_properties",
        input: r#"{
"name": "{{name}}"{{#if email}},
"email": "{{email}}"{{/if}}{{#if age}},
"age": {{age}}{{/if}}
}"#,
        expected: r#"{
  "name": "{{name}}"{{#if email}},
  "email": "{{email}}"{{/if}}{{#if age}},
  "age": {{age}}{{/if}}
}"#,
        description: "Format conditional JSON properties with mustache",
    },
    TestCase {
        name: "mustache_in_array",
        input: r#"[
{{#items}}
"{{.}}"{{#unless @last}},{{/unless}}
{{/items}}
]"#,
        expected: r#"[
  {{#items}}
  "{{.}}"{{#unless @last}},{{/unless}}
  {{/items}}
]"#,
        description: "Format mustache sections within JSON arrays",
    },
];

pub const COMMENT_TESTS: &[TestCase] = &[
    TestCase {
        name: "json5_line_comment",
        input: r#"{
  "name": "test", // This is a comment
  "value": 123
}"#,
        expected: r#"{
  "name": "test", // This is a comment
  "value": 123
}"#,
        description: "Preserve JSON5 line comments",
    },
    TestCase {
        name: "json5_block_comment",
        input: r#"{
  /* This is a
     block comment */
  "name": "test"
}"#,
        expected: r#"{
  /* This is a
     block comment */
  "name": "test"
}"#,
        description: "Preserve JSON5 block comments",
    },
    TestCase {
        name: "mustache_comment",
        input: r#"{{! This is a mustache comment }}
{
  "name": "{{name}}"
}"#,
        expected: r#"{{! This is a mustache comment }}
{
  "name": "{{name}}"
}"#,
        description: "Preserve mustache comments",
    },
];

pub const EDGE_CASE_TESTS: &[TestCase] = &[
    TestCase {
        name: "empty_object",
        input: r#"{}"#,
        expected: r#"{}"#,
        description: "Handle empty objects",
    },
    TestCase {
        name: "empty_array",
        input: r#"[]"#,
        expected: r#"[]"#,
        description: "Handle empty arrays",
    },
    TestCase {
        name: "single_property_object",
        input: r#"{"name":"test"}"#,
        expected: r#"{"name": "test"}"#,
        description: "Handle single property objects (keep inline)",
    },
    TestCase {
        name: "whitespace_preservation",
        input: r#"{
  "template": "{{#users}}


{{name}}


{{/users}}"
}"#,
        expected: r#"{
  "template": "{{#users}}


  {{name}}


  {{/users}}"
}"#,
        description: "Preserve significant whitespace in templates",
    },
];

pub const REAL_WORLD_TESTS: &[TestCase] = &[
    TestCase {
        name: "api_response_template",
        input: r#"{
"status": "success",
"data": {
"users": [
{{#users}}
{
"id": {{id}},
"name": "{{name}}",
"email": "{{email}}"{{#if profile}},
"profile": {
"avatar": "{{profile.avatar}}",
"bio": "{{profile.bio}}"
}{{/if}}
}{{#unless @last}},{{/unless}}
{{/users}}
]
}
}"#,
        expected: r#"{
  "status": "success",
  "data": {
    "users": [
      {{#users}}
      {
        "id": {{id}},
        "name": "{{name}}",
        "email": "{{email}}"{{#if profile}},
        "profile": {
          "avatar": "{{profile.avatar}}",
          "bio": "{{profile.bio}}"
        }{{/if}}
      }{{#unless @last}},{{/unless}}
      {{/users}}
    ]
  }
}"#,
        description: "Format a realistic API response template",
    },
    TestCase {
        name: "config_template",
        input: r#"{
"database": {
"host": "{{db_host}}",
"port": {{db_port}},
"name": "{{db_name}}"
},
"features": [
{{#features}}
"{{name}}"{{#unless @last}},{{/unless}}
{{/features}}
],
"settings": {
{{#each settings}}
"{{@key}}": {{#if (eq type "string")}}"{{value}}"{{else}}{{value}}{{/if}}{{#unless @last}},{{/unless}}
{{/each}}
}
}"#,
        expected: r#"{
  "database": {
    "host": "{{db_host}}",
    "port": {{db_port}},
    "name": "{{db_name}}"
  },
  "features": [
    {{#features}}
    "{{name}}"{{#unless @last}},{{/unless}}
    {{/features}}
  ],
  "settings": {
    {{#each settings}}
    "{{@key}}": {{#if (eq type "string")}}"{{value}}"{{else}}{{value}}{{/if}}{{#unless @last}},{{/unless}}
    {{/each}}
  }
}"#,
        description: "Format a configuration template with complex logic",
    },
];

pub fn all_test_cases() -> Vec<&'static TestCase> {
    let mut all_tests = Vec::new();
    all_tests.extend(BASIC_JSON5_TESTS);
    all_tests.extend(MUSTACHE_TESTS);
    all_tests.extend(MIXED_CONTENT_TESTS);
    all_tests.extend(COMMENT_TESTS);
    all_tests.extend(EDGE_CASE_TESTS);
    all_tests.extend(REAL_WORLD_TESTS);
    all_tests
}

pub fn write_test_files() -> std::io::Result<()> {
    let test_dir = Path::new("test_files");
    fs::create_dir_all(test_dir)?;

    for test_case in all_test_cases() {
        let input_file = test_dir.join(format!("{}_input.mustache_json5", test_case.name));
        let expected_file = test_dir.join(format!("{}_expected.mustache_json5", test_case.name));

        fs::write(input_file, test_case.input)?;
        fs::write(expected_file, test_case.expected)?;
    }

    Ok(())
}
