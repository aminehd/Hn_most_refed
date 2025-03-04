-- Create tables for hourly aggregates
CREATE TABLE IF NOT EXISTS hourly_aggregates (
  id SERIAL PRIMARY KEY,
  hour TIMESTAMP NOT NULL,
  source_name TEXT NOT NULL,
  item_count INTEGER NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index for faster queries by hour
CREATE INDEX IF NOT EXISTS hourly_aggregates_hour_idx ON hourly_aggregates(hour);

-- Create index for faster queries by source
CREATE INDEX IF NOT EXISTS hourly_aggregates_source_idx ON hourly_aggregates(source_name);

-- Function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically update the updated_at column
CREATE TRIGGER update_hourly_aggregates_updated_at
BEFORE UPDATE ON hourly_aggregates
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Create sample data for testing
INSERT INTO hourly_aggregates (hour, source_name, item_count)
VALUES 
  (NOW() - INTERVAL '1 hour', 'hacker_news', 42),
  (NOW() - INTERVAL '2 hours', 'hacker_news', 36),
  (NOW() - INTERVAL '3 hours', 'hacker_news', 51);