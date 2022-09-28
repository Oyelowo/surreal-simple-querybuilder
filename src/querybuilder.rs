use std::collections::HashMap;

pub struct QueryBuilder<'a> {
  segments: Vec<QueryBuilderSegment<'a>>,
  parameters: HashMap<&'a str, &'a str>,
  storage: Vec<String>,
}

impl<'a> QueryBuilder<'a> {
  pub fn new() -> Self {
    QueryBuilder {
      segments: Vec::new(),
      parameters: HashMap::new(),
      storage: Vec::new(),
    }
  }

  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new().create("Person:ee").build();
  ///
  /// assert_eq!(query, "CREATE Person:ee")
  /// ```
  pub fn create(mut self, node: &'a str) -> Self {
    self.add_segment_p("CREATE", node);

    self
  }

  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new().update("Person:ee").build();
  ///
  /// assert_eq!(query, "UPDATE Person:ee")
  /// ```
  pub fn update(mut self, node: &'a str) -> Self {
    self.add_segment_p("UPDATE", node);

    self
  }

  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new().select("ee:Person").build();
  ///
  /// assert_eq!(query, "SELECT ee:Person")
  /// ```
  pub fn select(mut self, node: &'a str) -> Self {
    self.add_segment_p("SELECT", node);

    self
  }

  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new().from("Person").build();
  ///
  /// assert_eq!(query, "FROM Person")
  pub fn from(mut self, node: &'a str) -> Self {
    self.add_segment_p("FROM", node);

    self
  }

  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new().select_many(&["ee:Person", "o:Order"]).build();
  ///
  /// assert_eq!(query, "SELECT ee:Person , o:Order")
  /// ```
  pub fn select_many(mut self, nodes: &[&'a str]) -> Self {
    self.add_segment("SELECT");
    self.join_segments(",", "", nodes, "");

    self
  }

  /// Adds the supplied query with a comma in front of it
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new().also(&"ee").build();
  ///
  /// assert_eq!(query, ", ee")
  /// ```
  pub fn also(mut self, query: &'a str) -> Self {
    self.add_segment_p(",", query);

    self
  }

  /// Adds the given segments, separated by the given `separator` and with a `prefix`
  /// and a `suffix` added to them too.
  ///
  /// # Example
  /// ```rs
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .join_segments(",", "set", &["handle", "id"], "")
  ///   .build();
  ///
  /// assert_eq!(query, "set handle , set id");
  /// ```
  #[allow(dead_code)]
  fn join_segments<T: Into<QueryBuilderSegment<'a>>>(
    &mut self, seperator: &'a str, prefix: &'a str, segments: &[T], suffix: &'a str,
  ) -> &mut Self
  where
    T: Copy,
  {
    let segments_count = segments.len();

    if segments_count <= 1 {
      for segment in segments {
        self.add_segment_ps(prefix, *segment, suffix);
      }

      return self;
    }

    for i in 0..segments_count - 1 {
      self.add_segment_ps(prefix, segments[i], suffix);
      self.add_segment(seperator);
    }

    self.add_segment_ps(prefix, segments[segments_count - 1], suffix);

    self
  }

  /// Starts a WHERE clause.
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .filter("handle = ?1")
  ///   .build();
  ///
  /// assert_eq!(query, "WHERE handle = ?1");
  /// ```
  pub fn filter(mut self, condition: &'a str) -> Self {
    self.add_segment_p("WHERE", condition);

    self
  }

  /// An alias for `QueryBuilder::filter`
  pub fn and_where(self, condition: &'a str) -> Self {
    self.filter(condition)
  }

  /// Starts a WHERE clause.
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .and("handle = ?1")
  ///   .build();
  ///
  /// assert_eq!(query, "AND handle = ?1");
  /// ```
  pub fn and(mut self, condition: &'a str) -> Self {
    self.add_segment_p("AND", condition);

    self
  }

  /// Starts a SET clause.
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .set("handle = ?1")
  ///   .build();
  ///
  /// assert_eq!(query, "SET handle = ?1");
  /// ```
  pub fn set(mut self, update: &'a str) -> Self {
    self.add_segment_p("SET", update);

    self
  }

  /// Starts a SET clause with many fields.
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .set_many(&["handle = $1", "password = $2"])
  ///   .build();
  ///
  /// assert_eq!(query, "SET handle = $1 , password = $2");
  /// ```
  pub fn set_many<T: Into<QueryBuilderSegment<'a>>>(mut self, updates: &[T]) -> Self
  where
    T: Copy,
  {
    self.add_segment("SET");
    self.join_segments(",", "", updates, "");

    self
  }

  /// Starts a FETCH clause,
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .fetch("author")
  ///   .build();
  ///
  /// assert_eq!(query, "FETCH author");
  /// ```
  pub fn fetch(mut self, field: &'a str) -> Self {
    self.add_segment_p("FETCH", field);

    self
  }

  /// Starts a FETCH clause with zero or more fields,
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .fetch_many(&["author", "projects"])
  ///   .build();
  ///
  /// assert_eq!(query, "FETCH author , projects");
  /// ```
  pub fn fetch_many<T: Into<QueryBuilderSegment<'a>>>(mut self, fields: &[T]) -> Self
  where
    T: Copy,
  {
    self.add_segment("FETCH");
    self.join_segments(",", "", fields, "");

    self
  }

  /// Queues a condition which allows the next statement to be ignored if
  /// `condition` is `false`.
  ///
  /// Conditions can be nested, the queue works as a LIFO queue.
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .select_many(&["1", "2"])
  ///   .if_then(false, |query| query
  ///     .select_many(&["3", "4"])
  ///     // will not run:
  ///     .if_then(true, |query| query
  ///       .select_many(&["5", "6"])
  ///     )
  ///   )
  ///   .if_then(true, |query| query
  ///     .select_many(&["7", "8"])
  ///   )
  ///   .build();
  ///
  /// assert_eq!(query, "SELECT 1 , 2 SELECT 7 , 8");
  /// ```
  pub fn if_then(self, condition: bool, action: fn(Self) -> Self) -> Self {
    if !condition {
      return self;
    }

    action(self)
  }

  /// Pushes raw text to the buffer
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .raw("foo bar")
  ///   .build();
  ///
  /// assert_eq!(query, "foo bar");
  /// ```
  pub fn raw(mut self, text: &'a str) -> Self {
    self.add_segment(text);

    self
  }

  /// Start a queue where all of the new pushed actions are separated by commas.
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .commas(|query| query
  ///     .raw("foo")
  ///     .raw("bar")
  ///   ).build();
  ///
  /// assert_eq!(query, "foo , bar");
  /// ```
  pub fn commas(mut self, action: fn(Self) -> Self) -> Self {
    let other = action(QueryBuilder::new());

    for (index, segment) in other.segments.into_iter().enumerate() {
      if index <= 0 {
        self.segments.push(segment);
      } else {
        self.add_segment(",");
        self.segments.push(segment);
      }
    }

    self
  }

  /// Start a LIMIT clause.
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  ///
  /// let page_size = 10.to_string();
  /// let query = QueryBuilder::new()
  ///   .limit(&page_size)
  ///   .build();
  ///
  /// assert_eq!(query, "LIMIT 10")
  ///
  /// ```
  pub fn limit(mut self, limit: &'a str) -> Self {
    self.add_segment_p("LIMIT", limit);

    self
  }

  /// Start a START AT clause.
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  ///
  /// let page_size = 10.to_string();
  /// let query = QueryBuilder::new()
  ///   .start_at(&page_size)
  ///   .build();
  ///
  /// assert_eq!(query, "START AT 10")
  ///
  /// ```
  pub fn start_at(mut self, offset: &'a str) -> Self {
    self.add_segment_p("START AT", offset);

    self
  }

  /// Add the given segment to the internal buffer. This is a rather internal
  /// method that is set public for special cases, you should prefer using the `raw`
  /// method instead.
  pub fn add_segment<T: Into<QueryBuilderSegment<'a>>>(&mut self, segment: T) -> &mut Self {
    let into = segment.into();

    if let QueryBuilderSegment::Str(s) = into {
      if s.is_empty() {
        return self;
      }
    }

    self.segments.push(into);

    self
  }

  fn add_segment_p<T: Into<QueryBuilderSegment<'a>>>(
    &mut self, prefix: &'a str, segment: T,
  ) -> &mut Self {
    self.add_segment(prefix).add_segment(segment)
  }

  fn add_segment_ps<T: Into<QueryBuilderSegment<'a>>>(
    &mut self, prefix: &'a str, segment: T, suffix: &'a str,
  ) -> &mut Self {
    self.add_segment_p(prefix, segment).add_segment(suffix)
  }

  /// Add a parameter and its value to the query that will be used to replace all
  /// occurences of `key` into `value` when the `build` method is called.
  ///
  /// **IMPORTANT** Do not use this for user provided data, the input is not sanitized
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let query = QueryBuilder::new()
  ///   .select("{{field}}")
  ///   .from("Account")
  ///   .param("{{field}}", "id")
  ///   .build();
  ///
  /// assert_eq!("SELECT id FROM Account", query);
  /// ```
  pub fn param(mut self, key: &'a str, value: &'a str) -> Self {
    self.parameters.insert(key, value);

    self
  }

  pub fn build(self) -> String {
    let mut output = self
      .segments
      .iter()
      .map(|s| match s {
        QueryBuilderSegment::Str(s) => s,
        QueryBuilderSegment::Ref(i) => &self.storage[*i][..],
      })
      .collect::<Vec<&str>>()
      .join(" ");

    for (key, value) in self.parameters {
      let key_size = key.len();

      while let Some(index) = output.find(key) {
        output.replace_range(index..index + key_size, value);
      }
    }

    output
  }

  /// Tell the current query builder to execute the [QueryBuilderSetObject] trait
  /// for the given `T` generic type.
  pub fn set_object<T: QueryBuilderSetObject>(self) -> Self
  where
    T: 'a,
  {
    T::set_querybuilder_object(self)
  }

  /// Tell the current querybuilder to hold the given string into its internal
  /// buffer and to return a reference to the newly held value a `QueryBuilder`
  /// can use.
  ///
  /// This function is particularely useful if some of your code is inside a
  /// short-lived scope such as a closure but you still need to add segments to
  /// the querybuilder. However the fact the querybuilder initially holds series
  /// of `&'a str` makes it impossible, this is where you can tell the builder
  /// to _hold_ the given string for you.
  pub fn hold(&mut self, string: String) -> QueryBuilderSegment<'a> {
    let i = self.storage.len();

    self.storage.push(string);

    QueryBuilderSegment::Ref(i)
  }
}

/// This trait allows you to easily and safely convert a type into a series of
/// statements. One such case being a series of `field = $field` statements.
///
/// # Example
/// ```rs
/// impl QueryBuilderSetObject for Account {
///  fn set_querybuilder_object<'a>(mut querybuilder: QueryBuilder<'a>) -> QueryBuilder {
///    let a = &[
///      querybuilder.hold(schema::handle.equals_parameterized()),
///      querybuilder.hold(schema::password.equals_parameterized()),
///      querybuilder.hold(schema::email.equals_parameterized()),
///      querybuilder.hold(schema::roles.equals_parameterized()),
///    ];
///
///    querybuilder.set_many(a)
///  }
/// }
/// ```
///
/// which can be used like so:
/// ```rs
/// let query = QueryBuilder::new()
///   .create("Account:John")
///   .set_object::<Account>()
///   .build();
///
/// assert_eq!(
///   "CREATE Account:John SET handle = $handle , password = $password , email = $email , roles = $roles",
///   query
/// );
/// ```
///
/// Refer to the `test.rs` file to a more complete example.
pub trait QueryBuilderSetObject {
  fn set_querybuilder_object<'b>(querybuilder: QueryBuilder<'b>) -> QueryBuilder<'b>;
}

#[derive(Clone, Copy)]
pub enum QueryBuilderSegment<'a> {
  Str(&'a str),
  Ref(usize),
}

impl<'a> From<&'a str> for QueryBuilderSegment<'a> {
  fn from(i: &'a str) -> Self {
    QueryBuilderSegment::Str(i)
  }
}

impl<'a> From<usize> for QueryBuilderSegment<'a> {
  fn from(i: usize) -> Self {
    QueryBuilderSegment::Ref(i)
  }
}
