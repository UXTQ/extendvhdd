#![cfg_attr(feature = "specialize", feature(build_hasher_simple_hash_one))]

use std::hash::{BuildHasher, Hash, Hasher};

use ahash::RandomState;
use criterion::*;
use fxhash::FxHasher;

fn gen_word_pairs() -> Vec<String> {
    let words: Vec<_> = r#"
a, ability, able, about, above, accept, according, account, across, act, action,
activity, actually, add, address, administration, admit, adult, affect, after,
again, against, age, agency, agent, ago, agree, agreement, ahead, air, all,
allow, almost, alone, along, already, also, although, always, American, among,
amount, analysis, and, animal, another, answer, any, anyone, anything, appear,
apply, approach, area, argue, arm, around, arrive, art, article, artist, as,
ask, assume, at, attack, attention, attorney, audience, author, authority,
available, avoid, away, baby, back, bad, bag, ball, bank, bar, base, be, beat,
beautiful, because, become, bed, before, begin, behavior, behind, believe,
benefit, best, better, between, beyond, big, bill, billion, bit, black, blood,
blue, board, body, book, born, both, box, boy, break, bring, brother, budget,
build, building, business, but, buy, by, call, camera, campaign, can, cancer,
candidate, capital, car, card, care, career, carry, case, catch, cause, cell,
center, central, century, certain, certainly, chair, challenge, chance, change,
character, charge, check, child, choice, choose, church, citizen, city, civil,
claim, class, clear, clearly, close, coach, cold, collection, college, color,
come, commercial, common, community, company, compare, computer, concern,
condition, conference, Congress, consider, consumer, contain, continue, control,
cost, could, country, couple, course, court, cover, create, crime, cultural,
culture, cup, current, customer, cut, dark, data, daughter, day, dead, deal,
death, debate, decade, decide, decision, deep, defense, degree, Democrat,
democratic, describe, design, despite, detail, determine, develop, development,
die, difference, different, difficult, dinner, direction, director, discover,
discuss, discussion, disease, do, doctor, dog, door, down, draw, dream, drive,
drop, drug, during, each, early, east, easy, eat, economic, economy, edge,
education, effect, effort, eight, either, election, else, employee, end, energy,
enjoy, enough, enter, entire, environment, environmental, especially, establish,
even, evening, event, ever, every, everybody, everyone, everything, evidence,
exactly, example, executive, exist, expect, experience, expert, explain, eye,
face, fact, factor, fail, fall, family, far, fast, father, fear, federal, feel,
feeling, few, field, fight, figure, fill, film, final, finally, financial, find,
fine, finger, finish, fire, firm, first, fish, five, floor, fly, focus, follow,
food, foot, for, force, foreign, forget, form, former, forward, four, free,
friend, from, front, full, fund, future, game, garden, gas, general, generation,
get, girl, give, glass, go, goal, good, government, great, green, ground, group,
grow, growth, guess, gun, guy, hair, half, hand, hang, happen, happy, hard,
have, he, head, health, hear, heart, heat, heavy, help, her, here, herself,
high, him, himself, his, history, hit, hold, home, hope, hospital, hot, hotel,
hour, house, how, however, huge, human, hundred, husband, I, idea, identify, if,
image, imagine, impact, important, improve, in, include, including, increase,
indeed, indicate, individual, industry, information, inside, instead,
institution, interest, interesting, international, interview, into, investment,
involve, issue, it, item, its, itself, job, join, just, keep, key, kid, kill,
kind, kitchen, know, knowledge, land, language, large, last, late, later, laugh,
law, lawyer, lay, lead, leader, learn, least, leave, left, leg, legal, less,
let, letter, level, lie, life, light, like, likely, line, list, listen, little,
live, local, long, look, lose, loss, lot, love, low, machine, magazine, main,
maintain, major, majority, make, man, manage, management, manager, many, market,
marriage, material, matter, may, maybe, me, mean, measure, media, medical, meet,
meeting, member, memory, mention, message, method, middle, might, military,
million, mind, minute, miss, mission, model, modern, moment, money, month, more,
morning, most, mother, mouth, move, movement, movie, Mr, Mrs, much, music, must,
my, myself, name, nation, national, natural, nature, near, nearly, necessary,
need, network, never, new, news, newspaper, next, nice, night, no, none, nor,
north, not, note, nothing, notice, now, n't, number, occur, of, off, offer,
office, officer, official, often, oh, oil, ok, old, on, once, one, only, onto,
open, operation, opportunity, option, or, order, organization, other, others,
our, out, outside, over, own, owner, page, pain, painting, paper, parent, part,
participant, particular, particularly, partner, party, pass, past, patient,
pattern, pay, peace, people, per, perform, performance, perhaps, period, person,
personal, phone, physical, pick, picture, piece, place, plan, plant, play,
player, PM, point, police, policy, political, politics, poor, popular,
population, position, positive, possible, power, practice, prepare, present,
president, pressure, pretty, prevent, price, private, probably, problem,
process, produce, product, production, professional, professor, program,
project, property, protect, prove, provide, public, pull, purpose, push, put,
quality, question, quickly, quite, race, radio, raise, range, rate, rather,
reach, read, ready, real, reality, realize, really, reason, receive, recent,
recently, recognize, record, red, reduce, reflect, region, relate, relationship,
religious, remain, remember, remove, report, represent, Republican, require,
research, resource, respond, response, responsibility, rest, result, return,
reveal, rich, right, rise, risk, road, rock, role, room, rule, run, safe, same,
save, say, scene, school, science, scientist, score, sea, season, seat, second,
section, security, see, seek, seem, sell, send, senior, sense, series, serious,
serve, service, set, seven, several, sex, sexual, shake, share, she, shoot,
short, shot, should, shoulder, show, side, sign, significant, similar, simple,
simply, 