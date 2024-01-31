SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: migrations; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA migrations;


--
-- Name: _address; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public._address AS (
	line_one text,
	line_two text,
	city text,
	state text,
	country text,
	postal_code text
);


--
-- Name: COLUMN _address.line_one; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public._address.line_one IS 'Address line 1 (e.g., street, PO Box, or company name).';


--
-- Name: COLUMN _address.line_two; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public._address.line_two IS 'Address line 2 (e.g., apartment, suite, unit, or building).';


--
-- Name: COLUMN _address.city; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public._address.city IS 'City, district, suburb, town, or village.';


--
-- Name: COLUMN _address.state; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public._address.state IS 'State, county, province, or region.';


--
-- Name: COLUMN _address.country; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public._address.country IS 'Two-letter country code (ISO 3166-1 alpha-2).';


--
-- Name: COLUMN _address.postal_code; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public._address.postal_code IS 'ZIP or postal code.';


--
-- Name: address; Type: DOMAIN; Schema: public; Owner: -
--

CREATE DOMAIN public.address AS public._address
	CONSTRAINT address_check CHECK (((((VALUE).line_one IS NOT NULL) AND ((VALUE).city IS NOT NULL) AND ((VALUE).state IS NOT NULL) AND ((VALUE).country IS NOT NULL) AND ((VALUE).postal_code IS NOT NULL)) OR (VALUE IS NULL)));


--
-- Name: gender; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.gender AS ENUM (
    'male',
    'female'
);


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: schema_migrations; Type: TABLE; Schema: migrations; Owner: -
--

CREATE TABLE migrations.schema_migrations (
    version character varying(128) NOT NULL
);


--
-- Name: attendance; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.attendance (
    session_id text NOT NULL,
    user_id text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: connect_group; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.connect_group (
    id text NOT NULL,
    no integer NOT NULL,
    name text,
    variant character(2),
    satellite_id text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN connect_group.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.connect_group.id IS 'Unique identifier of a connect_group (e.g., connect_group_01H7JNPD7J67AA5AD87Q4SZDF9).';


--
-- Name: COLUMN connect_group.no; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.connect_group.no IS 'Sequence number of a connect group (e.g., 1, 2, etc.).';


--
-- Name: COLUMN connect_group.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.connect_group.name IS 'Name of a connect group.';


--
-- Name: COLUMN connect_group.variant; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.connect_group.variant IS 'Variant of a connect group (e.g., J, S, T, W, A, B, C).';


--
-- Name: COLUMN connect_group.satellite_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.connect_group.satellite_id IS 'Satellite that the connect group belongs to.';


--
-- Name: COLUMN connect_group.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.connect_group.created_at IS 'Creation time of a connect group.';


--
-- Name: COLUMN connect_group.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.connect_group.updated_at IS 'Last updated time of a connect group.';


--
-- Name: connect_group_no_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.connect_group_no_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: connect_group_no_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.connect_group_no_seq OWNED BY public.connect_group.no;


--
-- Name: currency; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.currency (
    code character(3) NOT NULL,
    num integer NOT NULL,
    denominator integer NOT NULL,
    name text NOT NULL,
    countries text[] NOT NULL
);


--
-- Name: COLUMN currency.code; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.currency.code IS 'Currency code according to ISO4217 (e.g., USD, EUR, MYR, etc.).';


--
-- Name: COLUMN currency.countries; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.currency.countries IS 'An array of country code according to ISO3166-1 (e.g., MY, US, SG, etc.).';


--
-- Name: event; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.event (
    id text NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    type text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN event.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.event.id IS 'Unique identifier for an event (e.g., event_01H7JNPD7J67AA5AD87Q4SZDF9).';


--
-- Name: COLUMN event.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.event.created_at IS 'Creation time of a event.';


--
-- Name: COLUMN event.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.event.updated_at IS 'Last updated time of a event.';


--
-- Name: event_type; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.event_type (
    name text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN event_type.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.event_type.name IS 'Unique name of an event type (e.g., camp, conference, etc.).';


--
-- Name: COLUMN event_type.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.event_type.created_at IS 'Creation time of a event type.';


--
-- Name: COLUMN event_type.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.event_type.updated_at IS 'Last updated time of a event type.';


--
-- Name: form_field_type; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.form_field_type (
    type text NOT NULL,
    description text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: ministry; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.ministry (
    id text NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    department_id text NOT NULL,
    team_id text NOT NULL,
    satellite_id text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN ministry.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry.id IS 'Unique identifier of a ministry. (e.g., ministry_01H7JNPD7J67AA5AD87Q4SZDF9)';


--
-- Name: COLUMN ministry.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry.name IS 'Name of a ministry (e.g., Usher, Software Developer, etc.)';


--
-- Name: COLUMN ministry.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry.description IS 'Description of a ministry.';


--
-- Name: COLUMN ministry.department_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry.department_id IS 'Department that a ministry belongs to.';


--
-- Name: COLUMN ministry.team_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry.team_id IS 'Team that a ministry belongs to.';


--
-- Name: COLUMN ministry.satellite_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry.satellite_id IS 'Satellite that a ministry belongs to.';


--
-- Name: COLUMN ministry.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry.created_at IS 'Creation time of a ministry.';


--
-- Name: COLUMN ministry.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry.updated_at IS 'Last updated time of a ministry.';


--
-- Name: ministry_department; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.ministry_department (
    id text NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN ministry_department.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_department.id IS 'Unique identifier of a ministry department. (e.g., ministry_department_01H7JNPD7J67AA5AD87Q4SZDF9)';


--
-- Name: COLUMN ministry_department.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_department.name IS 'Name of a ministry department (e.g., People, Tech, etc.)';


--
-- Name: COLUMN ministry_department.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_department.description IS 'Description of a ministry department.';


--
-- Name: COLUMN ministry_department.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_department.created_at IS 'Creation time of a ministry department.';


--
-- Name: COLUMN ministry_department.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_department.updated_at IS 'Last updated time of a ministry department.';


--
-- Name: ministry_role; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.ministry_role (
    id text NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    weight integer NOT NULL
);


--
-- Name: COLUMN ministry_role.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_role.id IS 'The id of the ministry role (e.g., rol_2Nx8e5Tik0UnX4c1).';


--
-- Name: COLUMN ministry_role.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_role.name IS 'The name of the ministry role (e.g., HOD, Team Lead, etc.).';


--
-- Name: COLUMN ministry_role.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_role.description IS 'The description of the ministry role.';


--
-- Name: COLUMN ministry_role.weight; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_role.weight IS 'The weight of the ministry role.';


--
-- Name: ministry_team; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.ministry_team (
    id text NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN ministry_team.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_team.id IS 'Unique identifier of a ministry team. (e.g., ministry_team_01H7JNPD7J67AA5AD87Q4SZDF9)';


--
-- Name: COLUMN ministry_team.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_team.name IS 'Name of a ministry team (e.g., People Experience, Creative, etc.)';


--
-- Name: COLUMN ministry_team.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_team.description IS 'Description of a ministry team.';


--
-- Name: COLUMN ministry_team.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_team.created_at IS 'Creation time of a ministry team.';


--
-- Name: COLUMN ministry_team.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.ministry_team.updated_at IS 'Last updated time of a ministry team.';


--
-- Name: pastoral_role; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.pastoral_role (
    id text NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    weight integer NOT NULL
);


--
-- Name: COLUMN pastoral_role.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.pastoral_role.id IS 'The id of the pastoral role (e.g., rol_2Nx8e5Tik0UnX4c1).';


--
-- Name: COLUMN pastoral_role.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.pastoral_role.name IS 'The name of the pastoral role (e.g., CGL, OM, etc.).';


--
-- Name: COLUMN pastoral_role.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.pastoral_role.description IS 'The description of the pastoral role.';


--
-- Name: COLUMN pastoral_role.weight; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.pastoral_role.weight IS 'The weight of the pastoral role.';


--
-- Name: price; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.price (
    id text NOT NULL,
    event_id text NOT NULL,
    name text NOT NULL,
    fee integer NOT NULL,
    currency_code character(3) NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN price.event_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.price.event_id IS 'The corresponding identifier of the event.';


--
-- Name: registration; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.registration (
    id text NOT NULL,
    event_id text NOT NULL,
    name text NOT NULL,
    close_at timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: registration_form_field; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.registration_form_field (
    registration_id text NOT NULL,
    name text NOT NULL,
    label text NOT NULL,
    description text,
    type text NOT NULL,
    weight integer NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: registration_form_field_data; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.registration_form_field_data (
    registration_id text NOT NULL,
    name text NOT NULL,
    user_id text NOT NULL,
    data text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: registration_form_field_weight_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.registration_form_field_weight_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: registration_form_field_weight_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.registration_form_field_weight_seq OWNED BY public.registration_form_field.weight;


--
-- Name: satellite; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.satellite (
    id text NOT NULL,
    no integer NOT NULL,
    name text NOT NULL,
    address public.address NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN satellite.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.satellite.id IS 'Unique identifier of a satellite (e.g., satellite_01H7JNPD7J67AA5AD87Q4SZDF9).';


--
-- Name: COLUMN satellite.no; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.satellite.no IS 'Sequence number of a satellite (e.g., 1, 2, etc.).';


--
-- Name: COLUMN satellite.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.satellite.name IS 'Name of a satellite (e.g., Puchong).';


--
-- Name: COLUMN satellite.address; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.satellite.address IS 'Address of a satellite.';


--
-- Name: COLUMN satellite.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.satellite.created_at IS 'Creation time of a satellite.';


--
-- Name: COLUMN satellite.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.satellite.updated_at IS 'Last updated time of a satellite.';


--
-- Name: satellite_no_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.satellite_no_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: satellite_no_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.satellite_no_seq OWNED BY public.satellite.no;


--
-- Name: session; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.session (
    id text NOT NULL,
    event_id text NOT NULL,
    name text NOT NULL,
    description text,
    expected_attendees integer NOT NULL,
    start_at timestamp with time zone NOT NULL,
    end_at timestamp with time zone NOT NULL,
    actual_start_at timestamp with time zone,
    actual_end_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: user; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public."user" (
    id text NOT NULL,
    no integer NOT NULL,
    email text NOT NULL,
    email_verified boolean DEFAULT false NOT NULL,
    name text NOT NULL,
    username text,
    given_name text,
    family_name text,
    gender public.gender,
    ic_number text,
    phone_number text,
    phone_number_verified boolean DEFAULT false,
    nickname text,
    avatar_url text,
    address public.address,
    date_of_birth timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN "user".id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public."user".id IS 'Unique identifier of a user. (e.g., auth0|01H7JNPD7J67AA5AD87Q4SZDF9)';


--
-- Name: user_connect_group; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_connect_group (
    user_id text NOT NULL,
    connect_group_id text NOT NULL,
    user_role text NOT NULL
);


--
-- Name: user_ministry; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_ministry (
    user_id text NOT NULL,
    ministry_id text NOT NULL,
    user_role text NOT NULL
);


--
-- Name: user_no_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_no_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_no_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_no_seq OWNED BY public."user".no;


--
-- Name: connect_group no; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.connect_group ALTER COLUMN no SET DEFAULT nextval('public.connect_group_no_seq'::regclass);


--
-- Name: registration_form_field weight; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.registration_form_field ALTER COLUMN weight SET DEFAULT nextval('public.registration_form_field_weight_seq'::regclass);


--
-- Name: satellite no; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.satellite ALTER COLUMN no SET DEFAULT nextval('public.satellite_no_seq'::regclass);


--
-- Name: user no; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user" ALTER COLUMN no SET DEFAULT nextval('public.user_no_seq'::regclass);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: migrations; Owner: -
--

ALTER TABLE ONLY migrations.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: attendance attendance_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.attendance
    ADD CONSTRAINT attendance_pkey PRIMARY KEY (session_id, user_id);


--
-- Name: connect_group connect_group_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.connect_group
    ADD CONSTRAINT connect_group_pkey PRIMARY KEY (id);


--
-- Name: connect_group connect_group_satellite_id_no_variant_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.connect_group
    ADD CONSTRAINT connect_group_satellite_id_no_variant_key UNIQUE (satellite_id, no, variant);


--
-- Name: currency currency_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.currency
    ADD CONSTRAINT currency_pkey PRIMARY KEY (code);


--
-- Name: event event_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.event
    ADD CONSTRAINT event_pkey PRIMARY KEY (id);


--
-- Name: event_type event_type_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.event_type
    ADD CONSTRAINT event_type_pkey PRIMARY KEY (name);


--
-- Name: form_field_type form_field_type_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.form_field_type
    ADD CONSTRAINT form_field_type_pkey PRIMARY KEY (type);


--
-- Name: ministry_department ministry_department_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ministry_department
    ADD CONSTRAINT ministry_department_pkey PRIMARY KEY (id);


--
-- Name: ministry ministry_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ministry
    ADD CONSTRAINT ministry_pkey PRIMARY KEY (id);


--
-- Name: ministry_role ministry_role_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ministry_role
    ADD CONSTRAINT ministry_role_name_key UNIQUE (name);


--
-- Name: ministry_role ministry_role_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ministry_role
    ADD CONSTRAINT ministry_role_pkey PRIMARY KEY (id);


--
-- Name: ministry_team ministry_team_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ministry_team
    ADD CONSTRAINT ministry_team_pkey PRIMARY KEY (id);


--
-- Name: pastoral_role pastoral_role_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.pastoral_role
    ADD CONSTRAINT pastoral_role_name_key UNIQUE (name);


--
-- Name: pastoral_role pastoral_role_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.pastoral_role
    ADD CONSTRAINT pastoral_role_pkey PRIMARY KEY (id);


--
-- Name: price price_event_id_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.price
    ADD CONSTRAINT price_event_id_name_key UNIQUE (event_id, name);


--
-- Name: price price_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.price
    ADD CONSTRAINT price_pkey PRIMARY KEY (id);


--
-- Name: registration registration_event_id_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.registration
    ADD CONSTRAINT registration_event_id_name_key UNIQUE (event_id, name);


--
-- Name: registration_form_field_data registration_form_field_data_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.registration_form_field_data
    ADD CONSTRAINT registration_form_field_data_pkey PRIMARY KEY (registration_id, name, user_id);


--
-- Name: registration_form_field registration_form_field_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.registration_form_field
    ADD CONSTRAINT registration_form_field_pkey PRIMARY KEY (registration_id, name);


--
-- Name: registration_form_field registration_form_field_registration_id_weight_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.registration_form_field
    ADD CONSTRAINT registration_form_field_registration_id_weight_key UNIQUE (registration_id, weight);


--
-- Name: registration registration_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.registration
    ADD CONSTRAINT registration_pkey PRIMARY KEY (id);


--
-- Name: satellite satellite_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.satellite
    ADD CONSTRAINT satellite_pkey PRIMARY KEY (id);


--
-- Name: session session_event_id_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.session
    ADD CONSTRAINT session_event_id_name_key UNIQUE (event_id, name);


--
-- Name: session session_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.session
    ADD CONSTRAINT session_pkey PRIMARY KEY (id);


--
-- Name: user_connect_group user_connect_group_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_connect_group
    ADD CONSTRAINT user_connect_group_pkey PRIMARY KEY (user_id, connect_group_id);


--
-- Name: user user_email_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_email_key UNIQUE (email);


--
-- Name: user_ministry user_ministry_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_ministry
    ADD CONSTRAINT user_ministry_pkey PRIMARY KEY (user_id, ministry_id);


--
-- Name: user user_no_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_no_key UNIQUE (no);


--
-- Name: user user_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);


--
-- Name: user user_username_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_username_key UNIQUE (username);


--
-- Name: attendance attendance_session_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.attendance
    ADD CONSTRAINT attendance_session_id_fkey FOREIGN KEY (session_id) REFERENCES public.session(id);


--
-- Name: connect_group connect_group_satellite_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.connect_group
    ADD CONSTRAINT connect_group_satellite_id_fkey FOREIGN KEY (satellite_id) REFERENCES public.satellite(id) ON UPDATE CASCADE;


--
-- Name: event event_type_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.event
    ADD CONSTRAINT event_type_fkey FOREIGN KEY (type) REFERENCES public.event_type(name);


--
-- Name: ministry ministry_department_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ministry
    ADD CONSTRAINT ministry_department_id_fkey FOREIGN KEY (department_id) REFERENCES public.ministry_department(id) ON UPDATE CASCADE;


--
-- Name: ministry ministry_satellite_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ministry
    ADD CONSTRAINT ministry_satellite_id_fkey FOREIGN KEY (satellite_id) REFERENCES public.satellite(id) ON UPDATE CASCADE;


--
-- Name: ministry ministry_team_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ministry
    ADD CONSTRAINT ministry_team_id_fkey FOREIGN KEY (team_id) REFERENCES public.ministry_team(id) ON UPDATE CASCADE;


--
-- Name: price price_currency_code_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.price
    ADD CONSTRAINT price_currency_code_fkey FOREIGN KEY (currency_code) REFERENCES public.currency(code);


--
-- Name: price price_event_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.price
    ADD CONSTRAINT price_event_id_fkey FOREIGN KEY (event_id) REFERENCES public.event(id);


--
-- Name: registration registration_event_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.registration
    ADD CONSTRAINT registration_event_id_fkey FOREIGN KEY (event_id) REFERENCES public.event(id);


--
-- Name: registration_form_field_data registration_form_field_data_registration_id_name_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.registration_form_field_data
    ADD CONSTRAINT registration_form_field_data_registration_id_name_fkey FOREIGN KEY (registration_id, name) REFERENCES public.registration_form_field(registration_id, name);


--
-- Name: registration_form_field registration_form_field_registration_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.registration_form_field
    ADD CONSTRAINT registration_form_field_registration_id_fkey FOREIGN KEY (registration_id) REFERENCES public.registration(id);


--
-- Name: registration_form_field registration_form_field_type_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.registration_form_field
    ADD CONSTRAINT registration_form_field_type_fkey FOREIGN KEY (type) REFERENCES public.form_field_type(type);


--
-- Name: session session_event_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.session
    ADD CONSTRAINT session_event_id_fkey FOREIGN KEY (event_id) REFERENCES public.event(id);


--
-- Name: user_connect_group user_connect_group_connect_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_connect_group
    ADD CONSTRAINT user_connect_group_connect_group_id_fkey FOREIGN KEY (connect_group_id) REFERENCES public.connect_group(id) ON UPDATE CASCADE;


--
-- Name: user_connect_group user_connect_group_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_connect_group
    ADD CONSTRAINT user_connect_group_user_id_fkey FOREIGN KEY (user_id) REFERENCES public."user"(id) ON UPDATE CASCADE;


--
-- Name: user_connect_group user_connect_group_user_role_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_connect_group
    ADD CONSTRAINT user_connect_group_user_role_fkey FOREIGN KEY (user_role) REFERENCES public.pastoral_role(id) ON UPDATE CASCADE;


--
-- Name: user_ministry user_ministry_ministry_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_ministry
    ADD CONSTRAINT user_ministry_ministry_id_fkey FOREIGN KEY (ministry_id) REFERENCES public.ministry(id) ON UPDATE CASCADE;


--
-- Name: user_ministry user_ministry_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_ministry
    ADD CONSTRAINT user_ministry_user_id_fkey FOREIGN KEY (user_id) REFERENCES public."user"(id) ON UPDATE CASCADE;


--
-- Name: user_ministry user_ministry_user_role_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_ministry
    ADD CONSTRAINT user_ministry_user_role_fkey FOREIGN KEY (user_role) REFERENCES public.ministry_role(id) ON UPDATE CASCADE;


--
-- PostgreSQL database dump complete
--


--
-- Dbmate schema migrations
--

INSERT INTO migrations.schema_migrations (version) VALUES
    ('20240131155724');
