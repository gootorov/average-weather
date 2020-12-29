// this is, of course, for demonstration purposes only.
pub const INDEX: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Average weather</title>
    </head>
    <body>
        <p>Welcome to average weather!</p>
        <p>This is a RESTful service that requests weather forecasts from different sources and returns you the average data.</p>
        <p>Try the following:</p>
        <p><a href="http://localhost:8000/forecast/today/Moscow">http://localhost:8000/forecast/today/Moscow</a></p>
        <p><a href="http://localhost:8000/forecast/tomorrow/Moscow">http://localhost:8000/forecast/tomorrow/Moscow</a></p>
        <p><a href="http://localhost:8000/forecast/five-days/Moscow">http://localhost:8000/forecast/five-days/Moscow</a></p>
        <p><a href="http://localhost:8000/forecast/today/this_is_a_city_that_doesnt_exist">http://localhost:8000/forecast/today/this_is_a_city_that_doesnt_exist</a></p>
    </body>
</html>
"#;
