# Extension packaging options
EXTENSION := patina_db
DATA := sql/$(EXTENSION)--*.sql
REGRESS := regression

# Load PGXS.
PG_CONFIG := pg_config
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)
